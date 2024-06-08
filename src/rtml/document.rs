use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use super::html_elements::{HTMLElement, HTMLEnum};
use super::parse::{parse, ParserError};


/// A HTML Document.
pub struct HTMLDocument<'a>{
    attributes : HashMap<&'a str, &'a str>,
    head : HTMLElement<'a>,
    body : HTMLElement<'a>,
    pub(crate) custom_tags : Vec<HTMLElement<'a>>
}

impl<'a> HTMLDocument<'_>{

    fn recursive_sort(
        elements : Vec<HTMLEnum<'a>>,
        head : &mut HTMLElement<'a>,
        body : &mut HTMLElement<'a>,
        custom : &mut Vec<HTMLElement<'a>>,
        attr : &mut HashMap<&'a str, &'a str>){

        for elem in elements{
            //Recursively add stuff

            match elem{
                HTMLEnum::Comment(_) => {}
                HTMLEnum::Text(text) => {
                    body.add_text(text);
                }
                HTMLEnum::Element(mut html_node) => {
                    //Figure out if its contents go to head or body
                    //Do the same to children

                    match html_node.name().to_lowercase().as_str() {
                        "doctype" => { continue; }
                        "define" => { custom.push(html_node); continue; }
                        _ => {}
                    }

                    let children = html_node.children;
                    html_node.children = vec![];

                    let head_nodes : &[&str] = &[
                        "title",
                        "base",
                        "link",
                        "meta",
                        "style",
                        "script",
                        "noscript",
                        "template",
                        "object"
                    ];

                    let ignored = &["head", "body"];

                    let to_head : bool = head_nodes.contains(&html_node.name());
                    let ignore : bool = ignored.contains(&html_node.name());

                    if(to_head){
                        Self::recursive_sort(children, head, body, custom, attr);
                        if !ignore { head.add_child(html_node); }
                    }
                    else{
                        Self::recursive_sort(children, head, &mut html_node, custom, attr);
                        if !ignore { body.add_child(html_node); }
                    }
                }
            }

    }
    }

    /// Parses a HTML Document from the given string.
    /// Auto-fixes elements not being in head/body when they should
    pub fn from(document : &'a str) -> Result<HTMLDocument<'a>, ParserError> {
        let tokens = parse(document)?;
        return Ok(Self::from_tokens(tokens));
    }

    pub fn from_tokens(tokens : Vec<HTMLEnum<'a>>) -> HTMLDocument<'a>{
        let mut head = HTMLElement::new("head");
        let mut body = HTMLElement::new("body");
        let mut attributes: HashMap<&'a str, &'a str> = HashMap::new();
        let mut custom = vec![];

        Self::recursive_sort(tokens, &mut head, &mut body, &mut custom, &mut attributes);

        HTMLDocument{
            attributes,
            head,
            body,
            custom_tags: custom
        }
    }

}

impl Display for HTMLDocument<'_>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<!DOCTYPE html>")?;
        write!(f, "{}", self.head)?;
        write!(f, "{}", self.body)
    }
}
