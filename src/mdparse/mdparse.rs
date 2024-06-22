use crate::mdparse::mdparse::MarkdownToken::{BLOCKQUOTE, FORCED_EOL, ORDERED_LIST, UNORDERED_LIST};
use crate::rtml::html_elements::HTMLElementReference;
use crate::{digit, ordered_list_ender, title_marker, unordered_list_marker, whitespace};

#[derive(Debug)]
pub enum MarkdownToken<'a>{
    ///Marks a header. Number is between 1 and 6 inclusive
    TITLE(usize),

    /// An unordered list starter. Triggered by ([-][\w]+) at the start of a line (possibly after title)
    UNORDERED_LIST,

    ///An ordered list starter. Triggered by digit+ followed with a space (possibly after title)
    ORDERED_LIST,

    ///Contains the first backtick of the blockquote
    BLOCKQUOTE(&'a str),

    ///Contains a string of characters
    TEXT(&'a str),

    ///An inline HTML invocation
    INLINE_HTML(HTMLElementReference<'a>),

    ///A forced end of line. Happens when a line ends with two or more backspaces
    FORCED_EOL
}




fn is_url_definition<'a>(line : &'a str) -> Option<(&'a str, &'a str)>{

    //We are supposed to only feed a single line into this.
    assert_eq!(line.lines().count(), 1);

    //[identifier]: url
    let mut i = 0;

    let letters : Vec<char> = line.bytes().map(|x| x as char).collect();

    None
}

pub fn markdown_parse<'a>(document : &'a str) -> Vec<MarkdownToken<'a>>{

    let mut line_counter = 1;
    let mut result : Vec<MarkdownToken> = vec![];

    for mut line in document.lines(){

        let mut i = 0;
        let bytes  : Vec<char> = line.bytes().map(|x| x as char).collect();

        let mut title_strength = 0;


        // Identify titles
        while i < bytes.len() && matches!(bytes[i], title_marker!()) { //Identify the number of # at the start of the line
            title_strength += 1;
            i += 1;
        }
        if i >= bytes.len() || !matches!(bytes[i], whitespace!()){ //Make sure there's a whitespace after the title strength indicator, and the line doesnt end there
            title_strength = 0;
        }
        else {
            //Skip to after last whitespace
            i += (&bytes[i .. ]).iter().position(|x| !matches!(x, whitespace!())).unwrap_or(bytes.len() - i);
        }

        //Add the title token if necessary
        if title_strength != 0 {
            if title_strength <= 6 {
                result.push(MarkdownToken::TITLE(title_strength));
            } else{
                //TODO : Warning in case of extraneous # before title
            }
        }

        //Identify if there's a list elem
        if matches!(bytes[i], unordered_list_marker!()) && matches!(bytes[i + 1], whitespace!()){

            //Skip all following whitespaces
            i += (&bytes[i + 2 ..]).iter().position(|x| !matches!(x, whitespace!())).unwrap_or(bytes.len() - i);

            //Push unordered list token
            result.push(UNORDERED_LIST);
        }
        else if matches!(bytes[i], digit!()){ //Identify ordered list elems

            if let Some(pos) = (&bytes[i + 1..]).iter().position(|x| !matches!(x, digit!())){ //Find end of digits
                let rpos = pos + i + 1;
                if rpos < bytes.len() && matches!(bytes[rpos], ordered_list_ender!()) && rpos + 1 < bytes.len() && matches!(bytes[rpos + 1], whitespace!()){ //Check if its a point followed by a whitespace

                    //Find the end of the whitespaces
                    let whitespace_end = (&bytes[rpos + 2 ..]).iter().position(|x| !matches!(x, whitespace!())).unwrap_or(bytes.len() - rpos - 2) + rpos + 2;

                    //Add an ORDERED LIST token
                    i = whitespace_end;
                    result.push(ORDERED_LIST);
                }
            }
        }

        while i < bytes.len(){ //IGNORE WHITESPACES

            match (&bytes[i ..]).iter().position(|x| !matches!(x, whitespace!())){
                None => {
                    //MAYBE INSERT FORCED EOL, THEN BREAK;

                    if bytes.len() - i >= 2{
                        result.push(FORCED_EOL);
                    }

                    break;
                }
                Some(x) => {
                    let unskewed_index = x + i;
                    i = unskewed_index;
                }
            }

            //THEN TREAT THE NEXT SEQUENCE OF CHARACTERS

            //Check if its backquotes
            let backquotes_maybe = bytes.get(i .. i + 3);
            if backquotes_maybe.is_some() && String::from_utf8(backquotes_maybe.unwrap().iter().map(|&x| x as u8).collect()).unwrap() == r#"```"# {

                //We found some backticks
                result.push(BLOCKQUOTE(&line[i .. i + 1]));

                i += 3;
                continue;
            }

            //Check the following character
            match bytes[i]{

                _ => {}
            }

            //We got to the end, just leave
            i += 1;
        }

        //Identify if there's a blockquote


        //Make sure to only call that a title if its ended by a whitespace

        //Identify other beginning-of-line characters, such as list points


    }

    return result;
}