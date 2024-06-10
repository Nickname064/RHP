use crate::rtml::html_elements::{HTMLElement, HTMLElementReference, HTMLEnum};

/// Some html tags are self-closing and do not absolutely need an ending Slash
/// This is the case with `<br>`, for example (which can also be writted `<br/>`)
pub(crate) static SELF_CLOSABLE_TAGS: &[&str] = &[
    "are", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr", "command", "keygen", "menuitem", "frame", "!doctype",
];

/// Every non-self-closing element that CANNOT have children
/// and does not need parsing beyond looking for a tag end.
///
/// Not mandatory for a non-self closing tag that cannot have children,
/// but more optimized
static STERILE_TAGS: &[&str] = &["script"];

#[derive(Debug)]
pub enum ParserError {
    //TODO : Proper error handling
}

macro_rules! find_from {
    ($document:expr, $offset:expr, $pattern:expr) => {
        match &$document[$offset..].find($pattern) {
            None => None,
            Some(index) => Some(index + $offset),
        }
    };
}

///
/// Parses a string of tokens representing a HTML page
///
/// ## Arguments
///
/// * `document` - the string to be parsed.
///
/// ## Returns
/// A Vector of HTMLEnums, representing the contents of the HTML page.
pub fn parse<'a>(mut document: &'a str) -> Result<Vec<HTMLEnum<'a>>, ParserError>{
    parse_rec(document, None)
}

pub fn parse_rec<'a>(mut document: &'a str, parent_name : Option<&str>) -> Result<Vec<HTMLEnum<'a>>, ParserError> {
    //We start in regular data mode.
    //For now, what we see is text.
    //Let's try to find a tag

    let mut result: Vec<HTMLEnum<'a>> = vec![];
    let mut offset: usize = 0;
    // We search in the string from a certain position. That position is 'offset'

    //As long as there is text to read
    while offset < document.len() {
        //We found what might be a tag starter
        if let Some(index) = find_from!(document, offset, '<') {
            //Check if its a comment
            if matches!(document.get(index + 1..index + 4), Some("!--")) {
                //Comment tag

                if let Some(end) = find_from!(document, index + 4, "-->") {
                    result.push(
                        //Push the text before the comment
                        HTMLEnum::Text(&document[..index]),
                    );

                    result.push(
                        //Push the comments contents into the result
                        HTMLEnum::Comment(&document[index + 4..end]),
                    );

                    document = &document[end + 3..]; //Crop the comment end
                    continue; //Restart the process
                } else {
                    result.push(
                        //Push the text before the comment
                        HTMLEnum::Text(&document[..index]),
                    );

                    result.push(
                        //Push the rest of the document as a comment
                        HTMLEnum::Comment(&document[index + 3..]),
                    );

                    document = ""; //Flush the entire document into the returned comment
                    offset = 0; //Reset the offset

                    continue; //Restart the process
                }
            }
            //Check if it's a DOCTYPE
            else {
                let mut name: Option<&str> = None;

                match document.get(index + 1..index + 9) {
                    Some(x) if x.to_lowercase() == "!doctype" => {
                        //Names are case-insensitive
                        name = Some("!doctype");

                        document = &document[index + 9..]; //Cut off the title
                        offset = 0; //Reset the reading offset
                    }
                    _ => {
                        //Try to parse the name

                        //1.Test if the 1st char matches the conditions for the name
                        //document[index] = '<'

                        if find_from!(
                            document,
                            index + 1,
                            |x| matches!(x, 'A'..='Z' | 'a'..='z' | '_')
                        ) == Some(index + 1)
                        {
                            //First character matches

                            //Lets see up until where the string matches
                            match find_from!(
                                document,
                                index + 2,
                                |x| !matches!(x, 'A'..='Z' | 'a'..='z' | '_' | '-' | '0' ..= '9')
                            ) {
                                Some(i) => {
                                    //Found a name end
                                    name = Some(&document[index + 1..i]); //Transfer the name
                                    document = &document[i..]; //Crop the tag start from the document
                                    offset = 0; //Reset the reading offset
                                }
                                None => {
                                    todo!("UnterminatedTagName");
                                }
                            }
                        }
                    }
                }

                if name.is_some() {
                    let (doc, tag_info) = parse_tag(document, name.unwrap())?;
                    document = doc;
                    offset = 0;
                    result.push(HTMLEnum::Element(tag_info)); //Add the tag to the result
                } else {
                    //It's not a tag, remember to start reading after that element next time
                    offset += 1;
                }
            }
        } else {
            //No tag starter, this is only text

            result.push(HTMLEnum::Text(document));

            document = ""; //Flush document

            return Ok(result);
        }
    }

    if offset != 0 {
        //No tag starter, this document is only text
        result.push(HTMLEnum::Text(document));

        document = ""; //Flush document
    }

    return Ok(result);
}

///Returns (rest of document, parsed element)
fn parse_tag<'a>(
    mut document: &'a str,
    name: &'a str,
) -> Result<(&'a str, HTMLElementReference<'a>), ParserError> {
    enum Mode {
        None,
        Alpha(usize),
        String(usize),
        Closed,
    }

    let res_reference = HTMLElement::new();
    let mut res = res_reference.borrow_mut();
    res.name = name;

    let mut stored_attr: Option<&str> = None; //Stored attribute name
    let mut mode: Mode = Mode::None;

    let mut html_encoding_from: Option<usize> = None;

    let mut equaled: bool = false; //An equal sign was parsed, but an attribute value has not been parsed yes
    let mut quote_symbol = '"'; //The last symbol used to start a string

    let mut self_closed: bool = SELF_CLOSABLE_TAGS.contains(&&*name.to_lowercase()); //Is the tag self-closed ? (like <br>)
    let sterile: bool = STERILE_TAGS.contains(&&*name.to_lowercase()); //Is the tag unable to have children (like script) ? (here for optimization)

    for (index, char) in document.char_indices() {
        if html_encoding_from.is_some() {
            //This goes above the regular checking loop.

            //Skip to the end of the HTML_encoding character
            if char != ';' {
                continue;
            }

            //For now we discard the character
            //In later iterations we could decide to check it
            html_encoding_from = None;
            continue;
        }

        match mode {
            Mode::Closed => {
                match char {
                    '>' => {
                        //Only self-closed elements don't look for children.

                        document = &document[index + 1..];

                        if self_closed {
                            drop(res);
                            return Ok((document, res_reference));
                        } else {
                            let end_tag = format!("</{}>", name);
                            match document.find(&end_tag) {
                                None => {
                                    //No end found, close the tag and parse the children
                                    //Doing this allows for unclosed tags in a document
                                    // ex : <p> THIS IS A PARAGRAPH
                                    // gets turned into <p> THIS IS A PARAGRAPH</p>

                                    if sterile {
                                        res.add_text(document);
                                    } else {
                                        //Parse the children
                                        res.add_children(parse_rec(document, Some(res.name))?);
                                    }

                                    //Because the rest of the document has been parsed,
                                    //the return document is always empty
                                    drop(res);
                                    return Ok(("", res_reference));
                                }
                                Some(index) => {
                                    //We can only parse what's between here and the tag closure

                                    let recdoc = &document[..index];

                                    if sterile {
                                        //GO faster and avoid parsing the contents of certain tags (like script)
                                        res.add_text(recdoc);
                                    } else {
                                        //Parse the children
                                        res.add_children(parse_rec(recdoc, Some(res.name))?);
                                    }

                                    drop(res);
                                    return Ok((&document[index + end_tag.len()..], res_reference));
                                }
                            }
                        }
                    }

                    _ => {
                        //After a / in an opening tag, eg : <br/
                        todo!("Incorrect character after / closure")
                    }
                }
            }

            Mode::String(x) => {
                match char {
                    c if c == quote_symbol => {
                        let slice = &document[x + 1..index];

                        if equaled {
                            //A attribute must be stored
                            assert!(stored_attr.is_some(), "Incorrect '=' implementation");
                            res.attribute(stored_attr.unwrap(), slice);
                            stored_attr = None;
                            equaled = false;
                        } else {
                            //A string can both be a attribute or a value.
                            //Since its not a value, it must be an attribute

                            //Flushing the stored attribute it there is one
                            match stored_attr {
                                Some(attr) => {
                                    res.attribute(attr, "");
                                }
                                None => {}
                            };

                            //Storing the string contents
                            stored_attr = Some(slice);
                        }

                        mode = Mode::None;
                    }

                    _ => { /* IGNORED LOL */ }
                }
            }

            _ => {
                match char {
                    '>' => {
                        //Only self-closed elements don't look for children.

                        //Because it was not closed, we need to check the state of the machine

                        if let Mode::Alpha(x) = mode {
                            //If were at the end of a word : <div attr> ...

                            match stored_attr {
                                None => {
                                    res.attribute(&document[x..index], "");
                                }
                                Some(v) if equaled => {
                                    res.attribute(v, &document[x..index]);
                                    equaled = false;
                                }
                                Some(v) => {
                                    res.attribute(v, "");
                                    res.attribute(&document[x..index], "");
                                }
                            }
                        }

                        if equaled {
                            panic!("Unmatched = !");
                        }

                        document = &document[index + 1..];

                        if self_closed {
                            drop(res);
                            return Ok((document, res_reference));
                        } else {
                            let end_tag = format!("</{}>", name);
                            match document.find(&end_tag) {
                                None => {
                                    //No end found, close the tag and parse the children

                                    if sterile {
                                        res.add_text(document);
                                    } else {
                                        //Parse the children
                                        res.add_children(parse_rec(document, Some(res.name))?);
                                    }

                                    //Because the rest of the document has been parsed,
                                    //the return document is always empty
                                    drop(res);
                                    return Ok(("", res_reference));
                                }
                                Some(index) => {
                                    //We can only parse what's between here and the tag closure

                                    let recdoc = &document[..index];

                                    if sterile {
                                        //GO faster and avoid parsing the contents of certain tags (like script)
                                        res.add_text(recdoc);
                                    } else {
                                        //Parse the children
                                        res.add_children(parse_rec(recdoc, Some(res.name))?);
                                    }

                                    drop(res);
                                    return Ok((&document[index + end_tag.len()..], res_reference));
                                }
                            }
                        }
                    }

                    '/' => {
                        if let Mode::Alpha(x) = mode {
                            match stored_attr {
                                None => {
                                    res.attribute(&document[x..index], "");
                                }
                                Some(v) if equaled => {
                                    res.attribute(v, &document[x..index]);
                                    equaled = false;
                                }
                                Some(v) => {
                                    res.attribute(v, "");
                                    res.attribute(&document[x..index], "");
                                }
                            }
                        }

                        mode = Mode::Closed;
                        self_closed = true;
                    }

                    '"' | '\'' => {
                        //Strings can be both attribute and values.
                        //However, since they end themselves, they are easier to check

                        //Treat case WORD"STRING"
                        if let Mode::Alpha(x) = mode {
                            //We don't flush stored_attr, as no attribute should be stored at this point
                            //An assertion is enough
                            //Also, the = sign is supposed to end alpha words.
                            //We can also assert that equaled == false

                            assert!(stored_attr.is_none());
                            assert!(!equaled);

                            //This means that the word right before HAS to be a boolean attribute
                            res.attribute(&document[x..index], "");
                        }

                        quote_symbol = char;
                        mode = Mode::String(index);
                    }

                    '=' => {
                        if equaled {
                            todo!("Chaining EQUAL(=) symbols is not allowed");
                        }

                        //Transfer alpha
                        if let Mode::Alpha(x) = mode {
                            if let Some(word) = stored_attr {
                                if equaled {
                                    res.attribute(word, &document[x..index]);
                                    equaled = false;
                                } else {
                                    res.attribute(word, "");
                                    stored_attr = Some(&document[x..index])
                                }
                            } else {
                                stored_attr = Some(&document[x..index]);
                            }

                            mode = Mode::None;
                        }

                        equaled = true;
                    }

                    ' ' | '\r' | '\n' | '\t' => {
                        //Whitespaces only end an alphabetic word
                        //They dont yet push it as a attr

                        if let Mode::Alpha(x) = mode {
                            let slice = &document[x..index];

                            match stored_attr {
                                None => {
                                    //Store the new attribute
                                    stored_attr = Some(slice)
                                }
                                Some(word) => {
                                    if equaled {
                                        res.attribute(word, slice);
                                        equaled = false;
                                    } else {
                                        res.attribute(word, "");
                                        stored_attr = Some(slice)
                                    }
                                } //Flush the stored word as a boolean attr
                            }

                            //Store the new attribute
                        }
                    }

                    'A'..='Z' | 'a'..='z' => {
                        //Alphabetic
                        if let Mode::Alpha(_) = mode { /* NO OPERATION */
                        } else {
                            mode = Mode::Alpha(index);
                        }

                        //SPECIAL TREATMENT FOR HTML ENCODING HERE
                        //IF AN AMPERSAND APPEARS, IT IS EITHER AN HTML ENCODING, OR AN ERROR
                        //AS SUCH, WE CAN GAMBLE, AND IF IT TURNS OUT THAT THE CHARACTER IS INVALID,
                        //RETURN AN ERROR

                        if char == '&' {
                            html_encoding_from = Some(index);
                        }
                    }

                    '0'..='9' | '_' | '-' | '&' if matches!(mode, Mode::Alpha(_)) => {
                        continue;
                    }

                    other_char => {
                        panic!("Unexpected char : {}", other_char)
                    }
                }
            }
        }
    }

    todo!("Incorrect TAG syntax");
}
