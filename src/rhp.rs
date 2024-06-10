use crate::hqueries::HCombinedQuery;
use crate::rtml::html_elements::HTMLEnum::Element;
use crate::rtml::html_elements::{HTMLElement, HTMLElementReference, HTMLEnum};

const DIRECTIVE_NAMES: &[&str] = &[
    "define", //Define a custom element
             //"import", //Import another page's custom elements
             //"insert" //Insert the contents of another page here
];

#[derive(Clone, Debug)]
struct DEFINE<'a> {
    ///The name(identifier) of the custom tag
    pub tagname: String,

    ///The tag's contents
    pub contents: Vec<HTMLEnum<'a>>,
}

enum Directives<'a> {
    ///DEFINE a new custom tag
    DEFINE(DEFINE<'a>),
    //IMPORT{},
    //INSERT{}
}

/// Reads a HTMLToken Stream, and extracts the top-level special directives
fn filter_out_directives(stream: Vec<HTMLEnum>) -> (Vec<Directives>, Vec<HTMLEnum>) {
    let mut directives = vec![];
    let mut out = vec![];

    for element in stream {
        match element {
            Element(html_element) => {
                let borrow = html_element.borrow();

                if DIRECTIVE_NAMES.contains(&borrow.name().to_lowercase().as_str()) {
                    directives.push(parse_directives(html_element.clone()));
                    continue;
                } else {
                    out.push(Element(html_element.clone()));
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
fn parse_directives<'a>(directive: HTMLElementReference<'a>) -> Directives<'a> {
    let directive_borrow = directive.borrow();

    match directive_borrow.name().to_lowercase().as_str() {
        "define" => {
            //Parse define

            match directive_borrow
                .rec_html_children()
                .iter()
                .map(|x| x.borrow())
                .find(|x| x.name.to_lowercase().as_str() == "children" && x.children.len() != 0)
            {
                None => {}
                Some(x) => {
                    panic!(
                        "children tags are not allowed to have children of their own : {}, {:?}",
                        x, x.children
                    )
                }
            }

            let tagname = String::from(
                directive_borrow
                    .get_attribute("tagname")
                    .expect("define without tagname"),
            );
            let contents = directive_borrow.children.clone();

            Directives::DEFINE(DEFINE { tagname, contents })
        }
        unknown => {
            panic!("Error : Undefined directive name : {}", unknown);
        }
    }
}

impl<'a> DEFINE<'_> {

    /// Filters all the elements of list(containing the custom tag contents)
    /// to apply modifiers corresponding to the tag invocation.
    fn apply_to(
        &self,
        list: Vec<HTMLEnum<'a>>,
        tag_invocation: HTMLElementReference<'a>,
    ) -> Vec<HTMLEnum<'a>> {
        let mut parsed = vec![];

        for elem in list {
            match elem {
                Element(html) => {
                    let html_borrow = html.borrow();

                    if html_borrow.name == "children" { //Handle pasting children

                        let HTMLQuery : HCombinedQuery = HCombinedQuery::from_str(
                            html_borrow.attributes.iter().find(|(a,_)| a == &"select").unwrap_or(&("", "")).1
                        );

                        let mut possible_children = tag_invocation.borrow().children.clone();

                        let mut filtered_children = possible_children.into_iter().filter(|x|{
                            match x {
                                HTMLEnum::Element(html) => {
                                    HTMLQuery.matches(&html.borrow())
                                }
                                _ => { false }
                            }
                        }).collect();

                        drop(HTMLQuery);

                        parsed.append(&mut filtered_children);
                    } else {
                        let transformed_element = HTMLElement::new();
                        let mut element_borrow = transformed_element.borrow_mut();

                        element_borrow.name = html_borrow.name;
                        element_borrow.args = html_borrow.args.clone();
                        element_borrow.attributes = html_borrow.attributes.clone();

                        element_borrow.children = vec![];
                        element_borrow.add_children(self.apply_to(html_borrow.children.clone(), tag_invocation.clone()));

                        element_borrow.parent = html_borrow.parent.clone();

                        parsed.push(HTMLEnum::Element(transformed_element.clone()));
                    }
                }
                any => {
                    parsed.push(any);
                }
            }
        }

        parsed
    }

    /// Processed a stream of HTML tokens, to apply custom elements
    fn process_stream(
        html_stream: Vec<HTMLEnum<'a>>,
        custom_tags: &Vec<DEFINE<'a>>,
    ) -> Vec<HTMLEnum<'a>> {
        let mut processed = vec![];

        for node in html_stream {
            match node {
                Element(html_element) => {
                    if let Some(tag) = custom_tags
                        .iter()
                        .find(|elem| elem.tagname == html_element.borrow().name)
                    {
                        processed.append(&mut tag.apply_to(tag.contents.clone(), html_element));
                    } else {
                        let mut borrow = html_element.borrow_mut();
                        borrow.children =
                            Self::process_stream(borrow.children.clone(), custom_tags);
                        processed.push(Element(html_element.clone()));
                    }
                }
                any_other => {
                    processed.push(any_other);
                }
            }
        }

        processed
    }

    /// Taking in a DEFINE statement, and a list of the exising custom element,
    /// returns the dependency list of the given unprocessed DEFINE statement
    /// On a processed DEFINE statement, get_dependencies will return an empty vec
    fn get_dependencies(&self, custom_tags: &Vec<DEFINE<'a>>) -> Vec<String> {
        let mut dependencies = vec![];
        let mut possible_deps: Vec<&str> = custom_tags.iter().map(|x| x.tagname.as_str()).collect();

        for node in &self.contents {
            match node {
                Element(html) => {
                    for rec in html.borrow().rec_html_children().iter().chain(vec![&html.clone()]).map(|x| x.borrow()) {
                        //For each element in the tree
                        //Don't forget to consider the element itself

                        if let Some(index) = possible_deps.iter().position(|&x| x == rec.name) {
                            //If their name matches a tag, add dependency

                            dependencies.push(String::from(rec.name)); //Add the dependency
                            possible_deps.swap_remove(index); //You can't have the same dependency twice
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
    fn compile(custom_tags: Vec<DEFINE<'a>>) -> Vec<DEFINE<'a>> {
        //Build dependency list
        let mut dependencies: Vec<(DEFINE<'a>, Vec<String>)> = vec![];
        let mut clean: Vec<DEFINE<'a>> = vec![];

        for elem in &custom_tags {
            let elem_cp = elem.clone();
            let deps = elem.get_dependencies(&custom_tags);

            if deps.contains(&elem.tagname) {
                panic!("Infinitely recursive element : {}", elem_cp.tagname);
            }

            dependencies.push((elem_cp, deps))
        }

        loop {
            //Split the vectors into two groups
            let mut ripe = vec![];
            let mut unripe = vec![];

            // Pop an element
            while let Some((tag, deps)) = dependencies.pop() {
                if deps.is_empty() {
                    ripe.push(tag);
                } else {
                    unripe.push((tag, deps));
                }
            }

            let ripe_names: Vec<&String> = ripe.iter().map(|x| &x.tagname).collect();

            //Empty unripe vector
            while let Some((mut tag, mut deps)) = unripe.pop() {
                deps.retain(|x| !ripe_names.contains(&x));
                tag.contents = Self::process_stream(tag.contents, &ripe);
                dependencies.push((tag, deps));
            }

            clean.extend(ripe);

            if dependencies.is_empty() {
                break;
            }
        }

        clean
    }
}

pub fn process_document(tokens: Vec<HTMLEnum>) -> Vec<HTMLEnum> {
    let (directives, document_tokens) = filter_out_directives(tokens);

    let mut custom_tags = vec![];

    for dir in directives {
        let Directives::DEFINE(d) = dir;
        custom_tags.push(d);
    }

    custom_tags = DEFINE::compile(custom_tags);

    //APPLY CUSTOM ELEMENTS
    let processed = DEFINE::process_stream(document_tokens, &custom_tags);
    processed
}
