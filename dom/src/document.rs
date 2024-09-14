use super::html_elements::{HTMLNode, HTMLNodeRef, HTMLEnum, PrettyPrintable};
use std::fmt::{Display, Formatter};

/// A HTML Document.
pub struct HTMLDocument<'a> {
    doctype: Option<HTMLNodeRef<'a>>,
    head: HTMLNodeRef<'a>,
    body: HTMLNodeRef<'a>,
}

impl<'a> HTMLDocument<'a> {
    /// Parses a stream of standard HTML Tokens, and creates a fixed document from it,
    /// correcting head elements not being in the head, and body elements not being in the body
    fn recursive_sort(
        elements: Vec<HTMLEnum<'a>>,
        head: HTMLNodeRef<'a>,
        body: HTMLNodeRef<'a>,
    ) {
        let head_nodes: &[&str] = &[
            //These nodes are only valid in the head
            "title", "base", "link", "meta", "style", "script", "noscript", "template", "object",
        ];

        let ignored = &["head", "body"];

        for elem in elements {
            //Recursively add stuff

            match elem {
                HTMLEnum::Comment(_) => { /* IGNORE COMMENTS */ }
                HTMLEnum::Text(text) => {
                    body.borrow_mut().add_text(text);
                }
                HTMLEnum::Node(html_node) => {
                    //Figure out if its contents go to head or body
                    //Do the same to children

                    let children = std::mem::take(&mut html_node.borrow_mut().children);

                    let to_head: bool = head_nodes.contains(&html_node.borrow().name());
                    let ignore: bool = ignored.contains(&html_node.borrow().name());

                    if to_head {
                        Self::recursive_sort(children, head.clone(), body.clone());
                        if !ignore {
                            head.borrow_mut().add_child(html_node);
                        }
                    } else {
                        if !ignore {
                            Self::recursive_sort(children, head.clone(), html_node.clone());
                            body.borrow_mut().add_child(html_node);
                        } else {
                            Self::recursive_sort(children, head.clone(), body.clone());
                        }
                    }
                }
            }
        }
    }

    pub fn from_tokens(mut tokens: Vec<HTMLEnum<'a>>) -> HTMLDocument<'a> {
        let head = HTMLNode::new();
        let body = HTMLNode::new();

        let doctype: Option<HTMLNodeRef>;

        match tokens.first() {
            Some(HTMLEnum::Node(html)) if html.borrow().name.to_lowercase() == "!doctype" => {
                doctype = Some(html.clone());
                tokens.remove(0);
            }
            _ => {
                doctype = None;
            }
        }

        Self::recursive_sort(tokens, head.clone(), body.clone());

        head.borrow_mut().name = "head";
        body.borrow_mut().name = "body";

        HTMLDocument {
            doctype,
            head,
            body,
        }
    }
}

impl Display for HTMLDocument<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(d) = &self.doctype {
            write!(f, "{}", d.borrow())?;
        }

        write!(f, "{}", self.head.borrow())?;
        write!(f, "{}", self.body.borrow())
    }
}

impl PrettyPrintable for HTMLDocument<'_> {
    fn pretty_fmt_rec(&self, depth: usize) -> String {
        let mut buf = String::new();

        if let Some(d) = &self.doctype {
            buf += &format!(
                "{}{}\n",
                "\t".repeat(depth),
                d.borrow().pretty_fmt_rec(depth)
            );
        }

        buf += &self.head.borrow().pretty_fmt_rec(depth);
        buf += "\n";
        buf += &self.body.borrow().pretty_fmt_rec(depth);

        buf
    }
}
