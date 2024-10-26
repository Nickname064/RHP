use crate::errors::ParserErrorType::*;
use crate::errors::{ParserError, ParserErrorType};
use std::iter::Peekable;

use dom::html_elements::*;

//What characters can start a tag name
#[macro_export]
macro_rules! tag_name_starter_pattern {
    () => {
        'A'..='Z' | 'a'..='z' | '_'
    };
}

//What characters can be used inside a tag name (except first character)
#[macro_export]
macro_rules! tag_name_pattern {
    () => {
        'A'..='Z' | 'a'..='z' | '_' | '-' | '0' ..= '9'
    };
}

#[macro_export]
macro_rules! arg_name_starter_pattern {
    () => { 'A' ..= 'Z' | 'a' ..= 'z' };
}

#[macro_export]
macro_rules! arg_name_pattern {
    () => { 'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '-' | '_' | ':' | '.' };
}

#[macro_export]
macro_rules! val_name_starter_pattern {
    () => { 'A' ..= 'Z' | 'a' ..= 'z' };
}

#[macro_export]
macro_rules! val_name_pattern {
    () => { 'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '-' | '_' | ':' | '.' };
}

#[macro_export]
macro_rules! quotes_symbol {
    () => {
        '"' | '\''
    };
}

#[macro_export]
macro_rules! equal_sign {
    () => {
        '='
    };
}

#[macro_export]
macro_rules! html_encoded_start {
    () => {
        '&'
    };
}

#[macro_export]
macro_rules! tag_self_closer {
    () => {
        '/'
    };
}

/// The characters used to close a tag (>)
#[macro_export]
macro_rules! tag_closer {
    () => {
        '>'
    };
}

/// The characters used to open a tag (<)
#[macro_export]
macro_rules! tag_opener {
    () => {
        '<'
    };
}

/// A whitespace character (ignored)
#[macro_export]
macro_rules! whitespace {
    () => {
        ' ' | '\t' | '\r' | '\n'
    };
}

/// Starts special tags such as !DOCTYPE or !-- (comment) --
#[macro_export]
macro_rules! special_indicator {
    () => {
        '!'
    };
}

/// Some html tags can only have text as a child, meaning we can just skip characters until we find
/// the matching tag end.
pub const __QUICKPARSE: &[&str] = &["script"];

type Letter = char;
type CharIter = (usize, Letter);

pub(crate) fn consume_whitespaces<I>(mut characters: Peekable<I>) -> Peekable<I>
where
    I: Iterator<Item = CharIter> + Clone,
{
    peek_pos_til(&mut characters, |(_, x)| !matches!(x, whitespace!()));
    characters
}

/// Given a peekable iterator and a predicate, finds the first element that matches the predicate,
/// and return the number of non-matching elements seen.
/// Advances the iterator.
/// After execution, iterator.next() is the first element that matches the predicate
pub(crate) fn peek_pos_til<I, T, F>(iterator: &mut Peekable<I>, predicate: F) -> Option<usize>
where
    I: Iterator<Item = T>,
    F: Fn(&T) -> bool,
{
    let mut i = 0;

    loop {
        match iterator.peek() {
            None => {
                return None;
            }
            Some(x) if predicate(x) => {
                return Some(i);
            }
            _ => {
                i += 1;
                iterator.next();
            }
        }
    }
}

/// Given a html-formatted string starting with a tag name, extracts the name and returns a slice.
/// Works with a peekable iterator of (index, letter) instead of a raw string
/// Returns an error if the first letter is not a valid name start, or if the EOF is reached before the end of the name
pub(crate) fn consume_tag_name<I>(
    mut characters: Peekable<I>,
) -> Result<(Peekable<I>, usize, usize), ParserError>
where
    I: Iterator<Item = CharIter> + Clone,
{
    let from: usize;

    match characters.peek() {
        Some((i, tag_name_starter_pattern!())) => {
            from = *i;
            characters.next();
        }
        Some((i, _)) => {
            return Err(ParserError {
                char: *i,
                error_type: InvalidName,
            })
        }
        None => {
            return Err(ParserError {
                char: 0,
                error_type: UnexpectedEOF,
            })
        }
    }

    let mut to: usize = from + 1;

    loop {
        match characters.peek() {
            None => {
                return Err(ParserError {
                    char: 0,
                    error_type: UnexpectedEOF,
                });
            }
            Some((_, tag_name_pattern!())) => {
                characters.next();
            }
            Some((i, _)) => {
                to = *i;
                break;
            }
        }
    }

    return Ok((characters, from, to));
}

/// Given a html-formatted string starting with a tag attribute, extracts the attribute and returns a slice.
/// Works with a peekable iterator of (index, letter) instead of a raw string
fn consume_attribute<I>(
    mut characters: Peekable<I>,
) -> Result<(Peekable<I>, (usize, usize)), ParserError>
where
    I: Iterator<Item = CharIter> + Clone,
{
    let mut quoted: Option<Letter> = None;
    let from: usize;
    let to: usize;

    match characters.nth(0) {
        Some((i, quotation_mark @ quotes_symbol!())) if quoted.is_none() => {
            quoted = Some(quotation_mark);
            from = i + 1;
        }
        Some((i, arg_name_starter_pattern!())) => {
            from = i;
        }
        Some((i, _)) => {
            return Err(ParserError {
                char: i,
                error_type: InvalidAttribute,
            })
        }
        None => {
            return Err(ParserError {
                char: 0,
                error_type: UnexpectedEOF,
            })
        }
    }

    let p: Option<usize> = match quoted {
        Some(quote_sign) => peek_pos_til(&mut characters, |(_, x)| *x == quote_sign),
        None => peek_pos_til(&mut characters, |(_, x)| !matches!(x, arg_name_pattern!())),
    };

    if p.is_none() {
        return Err(ParserError {
            char: 0,
            error_type: UnexpectedEOF,
        });
    }

    let (upper_bound, _) = characters.peek().unwrap();
    to = *upper_bound;

    if quoted.is_some() {
        characters.next();
    }

    return Ok((characters, (from, to)));
}

/// Given a html-formatted string starting with a tag attribute value, extracts the value and returns a slice.
/// Works with a peekable iterator of (index, letter) instead of a raw string
fn consume_value<I>(
    mut characters: Peekable<I>,
) -> Result<(Peekable<I>, (usize, usize)), ParserError>
where
    I: Iterator<Item = CharIter> + Clone,
{
    let mut quoted: Option<Letter> = None;
    let from: usize;
    let to: usize;

    match characters.nth(0) {
        Some((i, quotation_mark @ quotes_symbol!())) if quoted.is_none() => {
            quoted = Some(quotation_mark);
            from = i + 1;
        }
        Some((i, val_name_starter_pattern!())) => {
            from = i;
        }
        Some((i, _)) => {
            return Err(ParserError {
                char: i,
                error_type: InvalidValue,
            })
        }
        None => {
            return Err(ParserError {
                char: 0,
                error_type: UnexpectedEOF,
            })
        }
    }

    let p: Option<usize> = match quoted {
        Some(quote_sign) => peek_pos_til(&mut characters, |(_, x)| *x == quote_sign),
        None => peek_pos_til(&mut characters, |(_, x)| !matches!(x, val_name_pattern!())),
    };

    if p.is_none() {
        return Err(ParserError {
            char: 0,
            error_type: UnexpectedEOF,
        });
    }

    let (upper_bound, _) = characters.peek().unwrap();
    to = *upper_bound;

    if quoted.is_some() {
        characters.next();
    }
    return Ok((characters, (from, to)));
}

/// Given a html-formatted string starting with a tag attribute, extracts the attribute, its value, and returns them.
/// Works with a peekable iterator of (index, letter) instead of a raw string
fn consume_attr_value<I>(
    mut characters: Peekable<I>,
    document: &str,
) -> Result<(Peekable<I>, String, Option<String>), ParserError>
where
    I: Iterator<Item = CharIter> + Clone,
{
    let (char, (from, to)) = consume_attribute(characters)?;
    characters = char;
    let attribute = String::from(&document[from..to]);

    let value: Option<String>;

    match characters.peek() {
        Some((_, equal_sign!())) => {
            characters.next();
            let (char, (from, to)) = consume_value(characters)?;
            characters = char;

            value = Some(String::from(&document[from..to]));
        }
        Some(_) => value = None,
        None => {
            return Err(ParserError {
                char: 0,
                error_type: UnexpectedEOF,
            })
        }
    };

    Ok((characters, attribute, value))
}

// Closes a HTML Tag, putting all the elements in last_layer as children of that Tag.
// Takes into account self-closed elements
// Returns Ok(layer the closed tags belong to) if a tag was closed
// Returns Err(layer that was supposed to be the tag's children) if no viable parent is found in
// the layer_stack
fn fold(
    layer_stack: &mut Vec<(Vec<HTMLEnum>, HTMLNodeRef)>,
    mut last_layer: Vec<HTMLEnum>,
) -> Result<Vec<HTMLEnum>, Vec<HTMLEnum>> {
    match layer_stack.pop() {
        None => Err(last_layer),
        Some((mut contents, tag)) => {
            if !__SELF_CLOSED.contains(&&*tag.borrow().name().to_lowercase()) {
                let mut tagborrow = tag.borrow_mut();
                tagborrow.add_children(last_layer);
                drop(tagborrow);
                contents.push(HTMLEnum::Node(tag));
                Ok(contents)
            } else {
                contents.push(HTMLEnum::Node(tag));
                contents.append(&mut last_layer);
                Ok(contents)
            }
        }
    }
}

/*
 * Forcibly folds the entire HTML layer stack
 * layer_stack : list of opened layers, each layer being (contents, opened tag)
 * last_layer : list of elements in the last layer
 */
fn fold_all(
    layer_stack: &mut Vec<(Vec<HTMLEnum>, HTMLNodeRef)>,
    mut last_layer: Vec<HTMLEnum>,
) -> Vec<HTMLEnum> {
    loop {
        match fold(layer_stack, last_layer) {
            Err(x) => {
                return x;
            }
            Ok(new_stack) => {
                last_layer = new_stack;
            }
        }
    }
}

fn find_word<I>(source: &mut Peekable<I>, word: &str) -> Option<usize>
where
    I: Iterator<Item = CharIter> + Clone,
{
    let word_chars: Vec<char> = word.chars().collect();

    while let Some(&(index, _)) = source.peek() {
        if word_chars
            .iter()
            .zip(source.clone())
            .all(|(x, (_, y))| *x == y)
        {
            return Some(index);
        }
        source.next();
    }

    None // Word not found
}

/// Parses a HTML Document
///
/// ### Returns
/// A vector containing the top-level elements
///
/// ### Errors
/// Returns an error if anything is incorrect in the document grammar.
/// For more information, please refer to [ParserError]
pub fn parse_html(document: &str) -> Result<Vec<HTMLEnum>, ParserError> //TODO : Make some errors recoverable
{
    let mut source = document.char_indices().peekable();
    let mut layer_stack = vec![];
    let mut last_layer = vec![];

    let mut doctype_attributes: Vec<(String, Option<String>)> = vec![];

    let is_self_closable = |name: &str| __SELF_CLOSED.contains(&&*name.to_lowercase());
    let mut text_used = true;
    let mut text_start: usize = 0; //dummy default value. Is instantly overwritten on line 417

    loop {
        if text_used {
            text_start = match source.peek() {
                None => {
                    return Ok(fold_all(&mut layer_stack, last_layer));
                }
                Some((i, _)) => i.clone(),
            };

            text_used = true;
        }

        let tag_start = match source.find(|(_, x)| matches!(x, tag_opener!())) {
            None => {
                // No tag, meaning everything left in the document is text
                last_layer.push(HTMLEnum::Text(String::from(&document[text_start..])));

                // End the document there, and close unclosed parent tags
                return Ok(fold_all(&mut layer_stack, last_layer));
            }
            Some((i, _)) => i.clone(),
        };

        //Check if it's a start or an end
        match source.peek() {
            None => {
                //No more chars after the possible starter, this is a valid EOF, push the rest of the document as text
                last_layer.push(HTMLEnum::Text(String::from(&document[text_start..])));

                // End the document there, and close unclosed parent tags
                return Ok(fold_all(&mut layer_stack, last_layer));
            }
            Some((_i, _x)) => {
                let i = _i.clone();
                let x = _x.clone();

                match x {
                    tag_self_closer!() => {
                        // HANDLE TAG CLOSURES
                        source.next();
                        let (_source, from, to) = consume_tag_name(source)?;
                        source = _source;

                        //Push the text up to that point as text
                        if tag_start != text_start {
                            last_layer.push(HTMLEnum::Text(String::from(
                                &document[text_start..tag_start],
                            )));
                        }

                        //Try to see if it matches the previously open tag
                        let closer_name = &document[from..to];

                        //Check that the tag closer correctly closes
                        match source.next() {
                            None => {
                                // Reached EOF when tag closer '>' was expected
                                return Err(ParserError {
                                    char: 0,
                                    error_type: UnexpectedEOF,
                                });
                            }
                            Some((_, tag_closer!())) => {}
                            Some((i, _)) => {
                                return Err(ParserError {
                                    char: i,
                                    error_type: UnexpectedCharacter {
                                        expected: vec!['>'],
                                    },
                                })
                            }
                        }

                        loop {
                            match layer_stack.last() {
                                Some((_, last_node))
                                    if last_node.borrow().name() == closer_name =>
                                {
                                    //Matches the previous tag, let's close it
                                    last_layer = fold(&mut layer_stack, last_layer).unwrap();
                                    break;
                                }
                                Some((_, last_node))
                                    if is_self_closable(last_node.borrow().name()) =>
                                {
                                    //If the previous tag is self-closable, lets close it and try the one before that
                                    last_layer =
                                        fold(&mut layer_stack, last_layer).map_err(|_| {
                                            ParserError {
                                                char: i,
                                                error_type: UnmatchedClosingTag,
                                            }
                                        })?;
                                }
                                _ => {
                                    //Otherwise the tag is just unmatched
                                    return Err(ParserError {
                                        char: i,
                                        error_type: UnmatchedClosingTag,
                                    });
                                }
                            }
                        }
                    }
                    tag_name_starter_pattern!() => {
                        //TAG NAMES

                        //Push the text up to that point as text
                        if tag_start != text_start {
                            last_layer.push(HTMLEnum::Text(String::from(
                                &document[text_start..tag_start],
                            )));
                        }
                        text_used = true;

                        let (_source, from, to) = consume_tag_name(source)?;
                        source = _source;

                        let node = HTMLNode::new();
                        let mut node_borrow = node.borrow_mut();

                        node_borrow.name = String::from(&document[from..to]).to_lowercase();
                        let mut closed: bool = false;

                        // Parse tag attributes
                        loop {
                            source = consume_whitespaces(source);
                            match source.peek() {
                                None => {
                                    return Err(ParserError {
                                        char: 0,
                                        error_type: ParserErrorType::UnexpectedEOF,
                                    })
                                }
                                Some((_, tag_closer!())) => {
                                    source.next();
                                    break;
                                }

                                Some((i, _)) if closed => {
                                    return Err(ParserError {
                                        char: *i,
                                        error_type: UnexpectedCharacter {
                                            expected: vec!['>'],
                                        },
                                    });
                                }

                                Some((_, tag_self_closer!())) => {
                                    closed = true;
                                    source.next();
                                }
                                Some((_, _)) => {
                                    let (_source, attr, val) =
                                        consume_attr_value(source, document)?;
                                    source = _source;
                                    node_borrow.attribute(attr.to_string(), val);
                                }
                            }
                        }

                        // TODO: Implement sterile tags (like <script>) here

                        if __QUICKPARSE.contains(&node_borrow.name()) {
                            // Find matching closing tag

                            let i = match source.peek() {
                                None => {
                                    return Err(ParserError {
                                        char: 0,
                                        error_type: ParserErrorType::UnexpectedEOF,
                                    })
                                }
                                Some((i, _)) => i.clone(),
                            };

                            match find_word(&mut source, &format!("</{}>", node_borrow.name())) {
                                None => {
                                    // The rest of the document is text as a child
                                    node_borrow.add_text(String::from(&document[i..]));
                                    drop(node_borrow);
                                    last_layer.push(HTMLEnum::Node(node));
                                    return Ok(fold_all(&mut layer_stack, last_layer));
                                }
                                Some(index) => {
                                    // Consume the next index- characters
                                    node_borrow.add_text(String::from(&document[i..index]));
                                    drop(node_borrow);
                                }
                            }
                        } else {
                            // We're done editing its attributes, so we can drop the mutable borrow
                            drop(node_borrow);
                        }

                        if !closed {
                            // If a node is not self-closed, then all the nodes until the tag end
                            // should be its children.
                            //
                            // We push the node on the stack to serve as a parent

                            layer_stack.push((last_layer, node));
                            last_layer = vec![];
                        } else {
                            // If a node is self-closed, then we can just consider it complete here
                            last_layer.push(HTMLEnum::Node(node));
                        }

                        continue;
                    }
                    special_indicator!() => {
                        //DOCTYPES and COMMENTS
                        source.next(); //Skip the !
                        let doc = "doctype";
                        let com = "--";

                        if com.chars().zip(source.clone()).all(|(x, (_, y))| x == y) {
                            //COMMENT
                            source.nth(com.len());

                            //Push the text up to that point as text
                            if tag_start != text_start {
                                last_layer.push(HTMLEnum::Text(String::from(
                                    &document[text_start..tag_start],
                                )));
                            }

                            match find_word(&mut source.clone(), "-->") {
                                None => {
                                    //Unterminated comment, push the entire document
                                    last_layer
                                        .push(HTMLEnum::Comment(String::from(&document[i + 3..])));
                                    return Ok(fold_all(&mut layer_stack, last_layer));
                                }
                                Some(index) => {
                                    //Push the comment on the stack
                                    last_layer.push(HTMLEnum::Comment(String::from(
                                        &document[i + 3..index],
                                    )));
                                    source.nth(index);
                                    continue;
                                }
                            }
                        } else if doc.chars().zip(source.clone()).all(|(x, (_, y))| {
                            y.to_lowercase().count() == 1 && y.to_lowercase().nth(0).unwrap() == x
                        }) {
                            //DOCTYPE
                            source.nth(doc.len());

                            //Push the text up to that point as text
                            if tag_start != text_start {
                                last_layer.push(HTMLEnum::Text(String::from(
                                    &document[text_start..tag_start],
                                )));
                            }

                            loop {
                                source = consume_whitespaces(source);
                                let mut closed: bool = false;

                                match source.peek() {
                                    Some((_, tag_closer!())) => {
                                        source.next();
                                        break;
                                    }
                                    _ if closed => {
                                        todo!("Invalid char")
                                    }
                                    Some((_, tag_self_closer!())) => {
                                        closed = true;
                                    }
                                    _ => {
                                        let (_source, attr, val) =
                                            consume_attr_value(source, document)?;
                                        source = _source;
                                        doctype_attributes.push((attr, val));
                                    }
                                }
                            }
                        } else {
                            text_used = false;
                        }
                    }
                    _ => {
                        text_used = false;
                    }
                }
            }
        }
    }
}
