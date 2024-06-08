use HTMLEnum::Element;
use crate::rtml::document::HTMLDocument;
use crate::rtml::html_elements::{HTMLElement, HTMLEnum};
use crate::rtml::parse::{parse, ParserError};

const directive_names : &[&str] = &[
    "define", //Define a custom element
    //"import", //Import another page's custom elements
    //"insert" //Insert the contents of another page here
];

struct DEFINE<'a> {
    ///The name(identifier) of the custom tag
    pub tagname: String,

    ///The tag's contents
    pub contents: Vec<HTMLEnum<'a>>,
}

enum Directives<'a>{

    ///DEFINE a new custom tag
    DEFINE(DEFINE<'a>),
    //IMPORT{},
    //INSERT{}
}

/// Reads a HTMLToken Stream, and extracts the top-level special directives
fn filter_out_directives<'a>(stream : Vec<HTMLEnum<'a>>) -> (Vec<Directives<'a>>, Vec<HTMLEnum<'a>>){

    let mut directives = vec![];
    let mut out = vec![];

    for element in stream{
        match element{
            Element(html_element) => {

                if directive_names.contains(&html_element.name().to_lowercase().as_str()){
                    directives.push(parse_directives(html_element));
                    continue;
                } else {
                    out.push(Element(html_element));
                }
            }
            _ => {
                out.push(element);
            }
        }
    }

    return (directives, out);
}

///Parses a HTMLElement representing a directive
fn parse_directives(directive : HTMLElement) -> Directives {

    match directive.name().to_lowercase().as_str(){
        "define" => {
            //Parse define

            if directive
                .rec_html_children()
                .iter()
                .find(|x| x.name.to_lowercase().as_str() == "children" && x.children.len() != 0)
                .is_some(){
                panic!("Children tags are not allowed to have children of their own");
            }

            let tagname = String::from(directive.get_attribute("tagname").expect("define without tagname"));
            let contents = directive.children;

            Directives::DEFINE(DEFINE {
                tagname,
                contents
            })

        }
        unknown => { panic!("Error : Undefined directive name : {}", unknown); }
    }
}

impl<'a> DEFINE<'_>{

    /// Filters all the elements of list(containing the custom tag contents)
    /// to apply modifiers corresponding to the tag invocation.
    fn apply_to(&self, list : Vec<HTMLEnum<'a>>, mut tag_invocation : HTMLElement<'a>) -> Vec<HTMLEnum<'a>>{

        let mut parsed = vec![];

        for elem in list{
            match elem{
                Element(html) => {
                    if html.name == "children"{
                        parsed.append(&mut tag_invocation.children.clone());
                    }
                    else{
                        parsed.push(
                            Element(HTMLElement {
                                name: html.name,
                                args: html.args,
                                attributes: html.attributes,
                                children : self.apply_to(html.children, tag_invocation.clone())
                            })
                        );
                    }
                }
                any => { parsed.push(any); }
            }
        }

        parsed
    }
}

impl<'a> HTMLDocument<'_>{
    pub fn from_rhp(document : &'a str) -> Result<(HTMLDocument<'a>), ParserError>{

        let (directives, document_tokens) = filter_out_directives(parse(document)?);

        let mut custom_tags = vec![];

        for dir in directives{
            let Directives::DEFINE(d) = dir;
            custom_tags.push(d);
        }

        //REPLACE HERE
        let mut processed = vec![];

        for token in document_tokens{
            match token{
                Element(mut html) => {
                    match custom_tags.iter().find(|&tag| tag.tagname == html.name){
                        Some(tag) => {
                            processed.append(&mut tag.apply_to(tag.contents.clone(), html));
                        }
                        None => {
                            processed.push(Element(html));
                        }
                    }
                }
                any => { processed.push(any) }
            }
        }

        let document = HTMLDocument::from_tokens(processed);
        return Ok(document);
    }
}