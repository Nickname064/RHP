use crate::parser::parse::parse_html;
use std::fs;
use std::iter::Peekable;

mod dom;
mod marsec;
mod parser;

fn find_word<I>(source: &mut Peekable<I>, word: &str) -> Option<usize>
where
    I: Iterator<Item = (usize, char)> + Clone,
{
    let word_chars: Vec<char> = word.chars().collect();

    while let Some(&(index, c)) = source.peek() {
        if word_chars.iter().zip(source.clone()).all(|(x, (_, y))| *x == y){
            return Some(index);
        } source.next();
    }

    None // Word not found
}

fn main() {
    //println!("{:#?}", HQuery::from_str("dom.class#id.class2"));
    //println!("{:#?}", HCombinedQuery::from_str("div > dom > div div"));


    match fs::read_to_string(r#"C:\Users\wanth\Documents\GitHub\RHP\DEBUG.rhp"#) {
        Ok(str) => {
            match parse_html(&str){
                Ok(x) => { println!("{:#?}", x); }
                Err(n) => { println!("{:#?}", n); }
            }
            /*
                let processed = process_document(tokens);
                let document = HTMLDocument::from_tokens(processed);
            */

            //fs::write(r#"C:\Users\wanth\Documents\GitHub\RHP\Output.html"#, document.pretty_fmt());

            //println!("{}", document);
        }
        Err(_) => {
            panic!("Umm, what the sigma")
        }
    }

    //let dbg_str = "# 1. test ```HELLO WORLD```";
    //println!("{:?}", markdown_parse(dbg_str));
}

