use crate::rhp::process_document;
use crate::rtml::document::HTMLDocument;
use std::{fmt, fs, io};
use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;
use crate::hqueries::{HCombinedQuery, HQuery};
use crate::mdparse::mdparse::markdown_parse;
use crate::rtml::html_elements::PrettyPrintable;
use crate::rtml::reparse::{consume_attr_value, consume_attribute, consume_tag_name, consume_value, html_parse};


mod hqueries;
mod rhp;
mod rtml;
mod utility;
mod mdparse;

fn main() {

    //println!("{:#?}", HQuery::from_str("dom.class#id.class2"));
    //println!("{:#?}", HCombinedQuery::from_str("div > dom > div div"));

    match fs::read_to_string(r#"C:\Users\wanth\Documents\GitHub\RHP\DEBUG.rhp"#) {
        Ok(str) => {


            let tokens = html_parse(&str);
            println!("{:#?}", tokens);

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

#[cfg(test)]

#[test]
fn test_parse_1(){
    assert!(html_parse("<div></div>").is_ok());
}

#[test]
fn test_parse_incorrect(){
    assert!(html_parse("<h2></div>").is_err());
}