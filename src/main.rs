use crate::rhp::process_document;
use crate::rtml::document::HTMLDocument;
use std::{fmt, fs, io};
use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;
use crate::hqueries::{HCombinedQuery, HQuery};
use crate::rtml::html_elements::PrettyPrintable;

use crate::rtml::parse::parse;

mod hqueries;
mod rhp;
mod rtml;
mod utility;

fn main() {

    //println!("{:#?}", HQuery::from_str("dom.class#id.class2"));
    //println!("{:#?}", HCombinedQuery::from_str("div > dom > div div"));

    match fs::read_to_string(r#"C:\Users\wanth\Documents\GitHub\RHP\DEBUG.rhp"#) {
        Ok(str) => {
            let filtered = str.replace("\r", "").replace("\n", "").replace("\t", "");

            let tokens = parse(&filtered).expect("uh oh");
            println!("{:#?}", tokens);
            let document = HTMLDocument::from_tokens(process_document(tokens));

            fs::write(r#"C:\Users\wanth\Documents\GitHub\RHP\Output.html"#, document.pretty_fmt());

            //println!("{}", document);
        }
        Err(_) => {
            panic!("Umm, what the sigma")
        }
    }
}

#[cfg(test)]

#[test]
fn test_parse_1(){
    assert!(parse("<div></div>").is_ok());
}

#[test]
fn test_parse_incorrect(){
    assert!(parse("<h2></div>").is_err());
}