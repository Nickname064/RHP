use std::fs;
use crate::rtml::document::HTMLDocument;

mod rtml;
mod rhp;
mod hqueries;

fn main() {
    match fs::read_to_string(r#"C:\Users\wanth\Documents\GitHub\RHP\DEBUG.rhp"#){
        Ok(str) => {

            let doc = HTMLDocument::from_rhp(&str).unwrap();

            println!("{}", doc);
        }
        Err(_) => {
            panic!("Umm, what the sigma")
        }
    }
}
