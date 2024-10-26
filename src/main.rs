use dom::html_elements::PrettyPrintable;
use parser::parse::parse_html;
use std::fs;

fn main() {
    for elem in std::env::args().skip(1) {
        match fs::read_to_string(elem) {
            Ok(str) => match parse_html(&str) {
                Ok(x) => {
                    for elem in x {
                        println!("{}", elem.pretty_fmt());
                    }
                }
                Err(n) => {
                    println!("{:#?}", n);
                    println!("Source : {}", &str[n.char..n.char + 10])
                }
            },
            Err(_) => {
                panic!("Umm, what the sigma")
            }
        }
    }
}
