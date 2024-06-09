use std::fs;
use std::io::stdout;
use std::io::Write;
use crate::rtml::document::HTMLDocument;
use crate::rtml::parse::parse;

mod rtml;
mod rhp;
mod hqueries;

fn main() {
    match fs::read_to_string(r#"C:\Users\wanth\Documents\GitHub\RHP\DEBUG.rhp"#){
        Ok(str) => {

            let filtered = str.replace("\r", "")
                .replace("\n", "")
                .replace("\t", "");

            let tokens = parse(&filtered).expect("uh oh");

            for token in tokens{
                println!("{:#?}", token);
            }
        }
        Err(_) => {
            panic!("Umm, what the sigma")
        }
    }
}
