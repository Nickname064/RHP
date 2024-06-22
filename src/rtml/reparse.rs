use std::iter::Peekable;
use crate::rtml::html_elements::{HTMLElement, HTMLElementReference, HTMLEnum};
use crate::rtml::reparse::ParserErrorType::{InvalidAttribute, InvalidName, InvalidValue, UnexpectedCharacter, UnexpectedEOF, UnmatchedClosingTag};
use crate::whitespace;

/// Some html tags are self-closing and do not absolutely need an ending Slash
/// This is the case with `<br>`, for example (which can also be writted `<br/>`)
pub(crate) static SELF_CLOSABLE_TAGS: &[&str] = &[
    "are", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr", "command", "keygen", "menuitem", "frame", "!doctype",
];

#[derive(Debug)]
pub struct ParserError{
    char : usize,
    error_type : ParserErrorType
}

#[derive(Debug)]
pub enum ParserErrorType{
    InvalidAttribute,
    InvalidValue,
    InvalidName,
    UnexpectedEOF,
    UnmatchedClosingTag,
    UnexpectedCharacter{ expected : Vec<char> }
}

//What characters can start a tag name
macro_rules! tag_name_starter_pattern {
    () => {
        'A'..='Z' | 'a'..='z' | '_'
    };
}

//What characters can be used inside a tag name (except first character)
macro_rules! tag_name_pattern {
    () => {
        'A'..='Z' | 'a'..='z' | '_' | '-' | '0' ..= '9'
    };
}

macro_rules! arg_name_starter_pattern {
    () => { 'A' ..= 'Z' | 'a' ..= 'z' };
}

macro_rules! arg_name_pattern {
    () => { 'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '-' | '_' };
}

macro_rules! val_name_starter_pattern {
    () => { 'A' ..= 'Z' | 'a' ..= 'z' };
}

macro_rules! val_name_pattern {
    () => { 'A' ..= 'Z' | 'a' ..= 'z' | '0' ..= '9' | '-' | '_' };
}

macro_rules! quotes_symbol {
    () => {'"' | '\'' };
}

macro_rules! equal_sign{
 () => { '=' }
}

macro_rules! html_encoded_start{
    () => { '&' }
}

macro_rules! tag_self_closer {
    () => { '/' };
}

macro_rules! tag_closer {
    () => { '>' };
}

macro_rules! tag_opener {
    () => { '<' };
}

type Letter = char;
type CharIter = (usize, Letter);

type Itertype = Peekable<dyn Iterator<Item=CharIter>>;

fn consume_whitespaces<I>(mut characters : Peekable<I>) -> Peekable<I>
where I : Iterator<Item = CharIter> + Clone
{
    loop {
        match characters.peek() {
            Some((_, whitespace!())) => { characters.next(); }
            _ => { return characters; }
        }
    }
}

fn peek_pos_til<I, T, F>(iterator : &mut Peekable<I>, predicate : F) -> Option<usize>
where
    I : Iterator<Item=T>,
    F : Fn(&T) -> bool
{

    let mut i = 0;

    loop{
        match iterator.peek(){
            None => { return None; }
            Some(x) if predicate(x) => { return Some(i); }
            _ => { i += 1; iterator.next(); }
        }
    }
}

pub fn consume_tag_name<I>(mut characters : Peekable<I>) -> Result<(Peekable<I>, (usize, usize)), ParserError>
where I : Iterator<Item = CharIter> + Clone
{

    let from : usize;
    let to : usize;

    match characters.peek(){
        Some((i, tag_name_starter_pattern!())) => { from = *i; characters.next(); }
        Some( (i, _) ) => { return Err(ParserError{ char : *i, error_type : InvalidName }) }
        None => { return Err(ParserError{ char : 0, error_type : UnexpectedEOF }) }
    }

    let p = characters.clone().skip(1).position(|(i, x)| !matches!(x, tag_name_pattern!()));
    match p {
        None => { return Err(ParserError{ char : 0, error_type : UnexpectedEOF }) }
        Some(i) => {
            to = from + i + 2; //+1 because we are working with exclusive ranges
            if i != 0 { characters.nth(i); } //Consume all name letters
        }
    }


    return Ok((characters, (from, to)));
}

pub fn consume_attribute<I>(mut characters : Peekable<I>) -> Result<(Peekable<I>, (usize, usize)), ParserError>
where I : Iterator<Item = CharIter> + Clone
{

    let mut quoted : Option<Letter> = None;
    let from : usize;
    let to : usize;

    match characters.nth(0){
        Some((i, quotation_mark @ quotes_symbol!())) if quoted.is_none() => {
            quoted = Some(quotation_mark);
            from = i + 1;
        }
        Some((i, arg_name_starter_pattern!())) => { from = i; }
        Some( (i, _) ) => { return Err(ParserError{ char : i, error_type : InvalidAttribute }) }
        None => return Err(ParserError{ char : 0, error_type : UnexpectedEOF })
    }

    let p : Option<usize> = match quoted {
        Some(quote_sign) => {
            peek_pos_til(&mut characters, |(_, x)| *x == quote_sign)
        }
        None => {
            peek_pos_til(&mut characters, |(_, x)| !matches!(x, arg_name_pattern!()))
        }
    };

    if p.is_none(){
        return Err( ParserError{ char : 0, error_type : UnexpectedEOF } );
    }

    let (upper_bound, _) = characters.peek().unwrap();
    to = *upper_bound;

    if quoted.is_some(){ characters.next(); }

    return Ok( (characters, (from, to)) );
}

pub fn consume_value<I>(mut characters : Peekable<I>) -> Result<(Peekable<I>, (usize, usize)), ParserError>
where I : Iterator<Item = CharIter> + Clone
{

    let mut quoted : Option<Letter> = None;
    let from : usize;
    let to : usize;

    match characters.nth(0){
        Some((i, quotation_mark @ quotes_symbol!())) if quoted.is_none() => {
            quoted = Some(quotation_mark);
            from = i + 1;
        }
        Some((i, val_name_starter_pattern!())) => { from = i; }
        Some( (i, _) ) => { return Err(ParserError{ char : i, error_type : InvalidValue }) }
        None => return Err(ParserError{ char : 0, error_type : UnexpectedEOF })
    }

    let p : Option<usize> = match quoted {
        Some(quote_sign) => {
            peek_pos_til(&mut characters, |(_, x)| *x == quote_sign)
        }
        None => {
            peek_pos_til(&mut characters, |(_, x)| !matches!(x, val_name_pattern!()))
        }
    };

    if p.is_none(){
        return Err( ParserError{ char : 0, error_type : UnexpectedEOF } );
    }

    let (upper_bound, _) = characters.peek().unwrap();
    to = *upper_bound;

    if quoted.is_some(){ characters.next(); }
    return Ok( (characters, (from, to)) );
}


pub fn consume_attr_value<I>(mut characters : Peekable<I>, document : &str) -> Result<(Peekable<I>, &str, Option<&str>), ParserError>
where I : Iterator<Item = CharIter> + Clone
{
    let (char, (from, to)) = consume_attribute(characters)?;
    characters = char;
    let attribute = &document[from .. to];

    let value : Option<&str>;

    match characters.peek(){
        Some( (_, equal_sign!()) ) => {

            characters.next();
            let (char, (from, to)) = consume_value(characters)?;
            characters = char;

            value = Some(&document[from .. to])
        }
        Some(_) => { value = None }
        None => { return Err(ParserError{char : 0, error_type : UnexpectedEOF })}
    };


    Ok( (characters, attribute, value) )
}

fn parse_tag_contents<'a, I>(mut characters : Peekable<I>, document : &'a str) -> Result<(Peekable<I>, HTMLElementReference<'a>), ParserError>
where I : Iterator<Item = CharIter> + Clone
{

    let name : &'a str;
    let mut attributes_values = vec![];
    let mut children : Vec<HTMLEnum> = vec![];

    //Extract name
    let (iter, (from, to)) = consume_tag_name(characters)?;
    characters = iter;
    name = &document[from .. to];


    //Extract attribute-values
    loop {

        //Skip all whitespaces
        characters = consume_whitespaces(characters);

        let attribute : &'a str;

        //Check if the next character is an end of tag

        match characters.peek(){
            Some((_, tag_self_closer!())) => { //Self-closed tag

                match characters.nth(1){
                    Some( (_, tag_closer!()) ) => {}
                    Some( (i, _) ) => { return Err(ParserError{ char : i, error_type : UnexpectedCharacter { expected : vec!['>'] } }) }
                    None => { return Err(ParserError{ char : 0, error_type : UnexpectedEOF }) }
                }

                children = vec![];
                break;
            }
            Some((_, tag_closer!())) => { //End of opening tag
                characters.next();

                if SELF_CLOSABLE_TAGS.contains(&name){
                    children = vec![];
                } else {
                    let (enums, new_chars) = parse_html_rec(characters, document, Some(name))?;
                    children = enums;
                    characters = new_chars;
                }
                break;
            }
            Some( _ ) => {
                /* This should be an attribute-value pair. Look below */
                let (chars, arg, value) =  consume_attr_value(characters, document)?;
                characters = chars;
                attributes_values.push( (arg, value.unwrap_or("")) );

            }
            None => { break; }
        }
    }

    let result = HTMLElement::new();
    let mut borrow = result.borrow_mut();
    borrow.name = name;
    borrow.attributes = attributes_values;
    borrow.children = children;
    drop(borrow);

    return Ok((characters, result));
}

fn parse_html_rec<'a, I>(mut characters : Peekable<I>, document : &'a str, parent_name : Option<&'a str>) -> Result<(Vec<HTMLEnum<'a>>, Peekable<I>), ParserError>
where I : Iterator<Item = CharIter> + Clone
{
    let mut result = vec![];

    loop {

        let first_index : usize;

        match characters.peek(){ //Extract index of first character, to know what to slice in case there is nothing
            None => { return Ok((result, characters))}
            Some((i, _)) => { first_index = *i; }
        }

        let next_pos = match characters.position(|(i, x)| matches!(x, tag_opener!())){
            None => {
                result.push(HTMLEnum::Text(&document[first_index..]));
                return Ok((result, characters));
            }
            Some(x) => {x}
        };

        let next_char = { match characters.peek(){
            Some( o @ (i, _)) => {
                if *i != first_index{
                    result.push(HTMLEnum::Text(&document[first_index .. i - 1]));
                } *o
            }
            None => {
                result.push(HTMLEnum::Text(&document[first_index..]));
                return Ok((result, characters));
            }
        }};

        match next_char {
            (i, tag_self_closer!()) => {
                //This is clearly a closing tag

                characters.next(); //consume the closing tag symbol

                let name: &'a str;

                let (chars, (from, to)) = consume_tag_name(characters)?;  //Try parsing its name
                characters = chars;
                name = &document[from..to];

                match parent_name{
                    Some(x)  if x == name => {}
                    _ => { return Err(ParserError{ char : i - 1, error_type : UnmatchedClosingTag }) /*unmatched closing brace*/ }
                }

                //Then expect a closing brace >
                match characters.nth(0) {
                    None => { return Err( ParserError{ char : 0, error_type : UnexpectedEOF } ); }
                    Some((_, tag_closer!())) => { /* Consume char */ }
                    Some((i, _)) => { return Err( ParserError{ char : i, error_type : UnexpectedCharacter {expected : vec!['>']} } ); }
                }

                //We now expect the closing tag to match with our parent, otherwise something is wrong

                return Ok((result, characters));
            }
            _ => {
                //This might be an opening tag. Try reading it.
                let (new_char, elem) = parse_tag_contents(characters, document)?;
                characters = new_char;
                result.push(HTMLEnum::Element(elem));
            }
        }
    }
}

pub fn html_parse(document : &str) -> Result<Vec<HTMLEnum>, ParserError> {

    let mut characters = document.char_indices().peekable();
    let (res, _) = parse_html_rec(characters, document, None)?;
    Ok(res)
}
