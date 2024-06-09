

use crate::rtml::html_elements::{HTMLEnum};


/*
const directive_names : &[&str] = &[
    "define", //Define a custom element
    //"import", //Import another page's custom elements
    //"insert" //Insert the contents of another page here
];

#[derive(Clone)]
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

                let borrow = html_element.borrow();

                if directive_names.contains(&borrow.name().to_lowercase().as_str()){
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
fn parse_directives(directive : HTMLElementReference) -> Directives {

    match directive.name().to_lowercase().as_str(){
        "define" => {
            //Parse define

            match directive
                .rec_html_children()
                .iter()
                .find(|x| x.name.to_lowercase().as_str() == "children" && x.children.len() != 0) {
                None => {}
                Some(x) => {
                    panic!("children tags are not allowed to have children of their own : {}, {:?}", x, x.children)
                }
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
    fn apply_to(&self, list : Vec<HTMLEnum<'a>>, tag_invocation : HTMLElementReference<'a>) -> Vec<HTMLEnum<'a>>{

        let mut parsed = vec![];

        for elem in list{
            match elem{
                Element(html) => {

                    let html_borrow = html.borrow();

                    if html_borrow.name == "children"{
                        parsed.append(&mut tag_invocation.borrow().children.clone());
                    }
                    else{

                        let transformed_element = HTMLElement::new();
                        let mut element_borrow = transformed_element.borrow_mut();

                        element_borrow.name = html_borrow.name();
                        element_borrow.args = html_borrow.args();
                        element_borrow.attributes = html_borrow.attributes();
                        element_borrow.children = self.apply_to(html_borrow.children.clone(), tag_invocation.clone());
                        element_borrow.parent = html_borrow.parent;

                        parsed.push(transformed_element);
                    }
                }
                any => { parsed.push(any); }
            }
        }

        parsed
    }

    /// Processed a stream of HTML tokens, to apply custom elements
    fn process_stream(html_stream : Vec<HTMLEnum<'a>>, custom_tags : &Vec<DEFINE<'a>>) -> Vec<HTMLEnum<'a>>{

        let mut processed = vec![];

        for node in html_stream{
            match node{
                Element(mut html_element) => {
                    if let Some(tag) = custom_tags.iter().find(|elem| elem.tagname == html_element.name){
                        processed.append(&mut tag.apply_to(tag.contents.clone(), html_element));
                    }
                    else{
                        html_element.children = Self::process_stream(html_element.children, custom_tags);
                        processed.push(Element(html_element));
                    }

                }
                any_other => { processed.push(any_other); }
            }
        }

        processed
    }

    /// Taking in a DEFINE statement, and a list of the exising custom element,
    /// returns the dependency list of the given unprocessed DEFINE statement
    /// On a processed DEFINE statement, get_dependencies will return an empty vec
    fn get_dependencies(&self, custom_tags : &'a Vec<DEFINE<'a>>) -> Vec<String>{
        let mut dependencies = vec![];
        let mut possible_deps: Vec<&'a str> = custom_tags.iter().map(|x| x.tagname.as_str()).collect();

        for node in &self.contents{
            match node{
                Element(html) => {
                    for rec in html.rec_html_children(){
                        if let Some(index) = possible_deps.iter().position(|&x| x == rec.name) {
                            dependencies.push(String::from(rec.name));
                            possible_deps.swap_remove(index);
                        }
                    }
                }
                _any => {}
            }
        }

        dependencies
    }

    /// Replaces custom elements in defined custom elements.
    /// Allows for recursive custom elements
    fn compile(custom_tags: Vec<DEFINE<'a>>) -> Vec<DEFINE<'a>>{

        //Build dependency list
        let mut dependencies : Vec<(DEFINE<'a>, Vec<String>)> = vec![];
        let mut clean: Vec<DEFINE<'a>> = vec![];

        for elem in &custom_tags {
            let elem_cp = elem.clone();
            let deps = elem.get_dependencies(&custom_tags);

            if deps.contains(&elem.tagname){
                panic!("Infinitely recursive element : {}", elem_cp.tagname);
            }

            dependencies.push((elem_cp, deps))
        }



        loop {
            //Split the vectors into two groups
            let mut ripe = vec![];
            let mut unripe = vec![];

            // Pop an element
            while let Some((tag, deps)) = dependencies.pop(){
                if deps.is_empty(){
                    ripe.push(tag);
                } else{
                    unripe.push((tag, deps));
                }
            }

            let ripe_names : Vec<&String> = ripe.iter().map(|x| &x.tagname).collect();

            //Empty unripe vector
            while let Some((mut tag, mut deps)) = unripe.pop(){
                deps.retain(|x| !ripe_names.contains(&x));
                tag.contents = Self::process_stream(tag.contents, &ripe);
                dependencies.push((tag, deps));
            }

            clean.extend(ripe);

            if dependencies.is_empty(){ break; }
        }

        clean
    }
}

pub fn process_document(tokens : Vec<HTMLEnum>) -> Vec<HTMLEnum>{

    let (directives, document_tokens) = filter_out_directives(tokens);

    let mut custom_tags = vec![];

    for dir in directives{
        let Directives::DEFINE(d) = dir;
        custom_tags.push(d);
    }

    custom_tags = DEFINE::compile(custom_tags);

    //APPLY CUSTOM ELEMENTS
    let processed = DEFINE::process_stream(document_tokens, &custom_tags);
    processed
}

*/