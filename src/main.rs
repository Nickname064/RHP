use std::fs;
use std::iter::Peekable;
use parser::parse::parse_html;

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

