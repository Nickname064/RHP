
use std::fmt::{Display, Formatter};
use super::html_elements::{HTMLElement, HTMLElementReference, HTMLEnum};



/// A HTML Document.
pub struct HTMLDocument<'a>{
    head : HTMLElementReference<'a>,
    body : HTMLElementReference<'a>,
}

impl<'a> HTMLDocument<'a>{

    /// Parses a stream of standard HTML Tokens, and creates a fixed document from it,
    /// correcting head elements not being in the head, and body elements not being in the body
    fn recursive_sort(
        elements : Vec<HTMLEnum<'a>>,
        head : HTMLElementReference<'a>,
        body : HTMLElementReference<'a>){


        let head_nodes : &[&str] = &[ //These nodes are only valid in the head
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


        for elem in elements{
            //Recursively add stuff

            match elem{
                HTMLEnum::Comment(_) => { /* IGNORE COMMENTS */ }
                HTMLEnum::Text(text) => {
                    body.borrow_mut().add_text(text);
                }
                HTMLEnum::Element(html_node) => {
                    //Figure out if its contents go to head or body
                    //Do the same to children

                    let children = std::mem::take(&mut html_node.borrow_mut().children);

                    let to_head : bool = head_nodes.contains(&html_node.borrow().name());
                    let ignore : bool = ignored.contains(&html_node.borrow().name());

                    if to_head {
                        Self::recursive_sort(children, head.clone(), body.clone());
                        if !ignore { head.borrow_mut().add_child(html_node); }
                    }
                    else{
                        if !ignore {
                            Self::recursive_sort(children, head.clone(), html_node.clone());
                            body.borrow_mut().add_child(html_node);
                        }
                        else{
                            Self::recursive_sort(children, head.clone(), body.clone());
                        }
                    }
                }
            }

    }
    }

    pub fn from_tokens(tokens : Vec<HTMLEnum<'a>>) -> HTMLDocument<'a>{
        let head = HTMLElement::new();
        let body = HTMLElement::new();

        Self::recursive_sort(tokens, head.clone(), body.clone());

        head.borrow_mut().name = "head";
        body.borrow_mut().name = "body";

        HTMLDocument{
            head,
            body
        }
    }
}


impl Display for HTMLDocument<'_>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<!DOCTYPE html>")?;
        write!(f, "{}", self.head.borrow())?;
        write!(f, "{}", self.body.borrow())
    }
}
