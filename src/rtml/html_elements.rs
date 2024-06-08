use std::array::IntoIter;
use std::fmt::{Display, Formatter};
use std::io::Chain;
use crate::rtml::parse::SELF_CLOSABLE_TAGS;

#[derive(Debug, Clone)]
/// Either Text Contents or a [`HTMLElement`](HTMLElement)
pub enum HTMLEnum<'a>{
    Text(&'a str),
    Element(HTMLElement<'a>),
    Comment(&'a str)
}

#[derive(Debug, Clone)]
/// A HTML tag.
pub struct HTMLElement<'a>{
    ///The name of this tag.
    /// Ex: for a ```<div></div>```, `name` = "div"
    pub(crate) name : &'a str,

    ///Hermes Feature
    ///Empty for regular HTML Elements
    ///
    /// Used by custom HTML tags to pass arguments to the underlying functions
    /// No need to read the arguments by hand, as elements with tags must be compiled to regular elements before they can be served
    pub(crate) args : Vec<(&'a str, &'a str)>,

    ///This tag's attributes
    pub(crate) attributes : Vec<(&'a str, &'a str)>,

    ///This tag's contents, whether they be tags or text.
    pub(crate) children : Vec<HTMLEnum<'a>>,
}

impl<'a> HTMLElement<'a>{
    pub fn new(name : &'a str) -> HTMLElement<'a> {
        HTMLElement{
            name,
            args : vec![],
            attributes : vec![],
            children : vec![],
        }
    }
    pub fn attribute(&mut self, attribute : &'a str, value : &'a str) -> &HTMLElement<'a>{
        //Attributes are only meant to be added / modified, not removed

        if let Some(index) = self.attributes.iter().position(|(name, _)| *name == attribute){
            self.attributes[index] = (attribute, value);
        }
        else{
            //TODO : Sort insertion
            self.attributes.push((attribute, value));
        }
        self
    }
    pub fn argument(&mut self, arg_name : &'a str, value : &'a str) -> &HTMLElement<'a>{

        //Attributes are only meant to be added, not removed

        if let Some(index) = self.args.iter().position(|(name, _)| *name == arg_name){
            self.args[index] = (arg_name, value);
        }
        else{
            //TODO : Sort insertion
            self.attributes.push((arg_name, value));
        }
        self
    }
    pub fn add_child(&mut self, mut child: HTMLElement<'a>) -> &HTMLElement<'a>{
        self.children.push(HTMLEnum::Element(child));
        self
    }
    pub fn add_children(&mut self, children : Vec<HTMLEnum<'a>>) -> &HTMLElement<'a>{
        for child in children {
            self.children.push(child);
        }
        self
    }
    pub fn add_text(&mut self, text : &'a str) -> &HTMLElement<'a>{
        self.children.push(HTMLEnum::Text(text));
        self
    }

    //Getter methods
    pub fn name(&self) -> &str { &self.name }
    pub fn get_attribute(&self, name : &str) -> Option<&str>{
        match self.attributes.iter().find(|(key, attribute)| key.to_string() == name.to_string()){
            Some((key, val)) => {return Some(val)}
            None => {return None; }
        }
    }
    pub fn children(&self) -> &Vec<HTMLEnum> {
        &self.children
    }
    pub fn self_closing(&self) -> bool {
        SELF_CLOSABLE_TAGS.iter().find(|&&x| x == self.name).is_some()
    }
    pub fn rec_html_children(&self) -> Vec<&HTMLElement<'a>> {

        let mut res = vec![self];

        for child in &self.children{
            match child{
                HTMLEnum::Element(html) => {
                    res.append(&mut html.rec_html_children());
                }
                _ => {}
            }
        }

        return res;
    }
}



impl Display for HTMLEnum<'_>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            HTMLEnum::Text(str) => { write!(f, "{}", str) }
            HTMLEnum::Element(elem) => { write!(f, "{}", elem)}
            HTMLEnum::Comment(str) => { write!(f, "<!--{}-->", str) }
        }
    }
}
impl Display for HTMLElement<'_>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "<{}", self.name)?;

        for (attribute, value) in &self.attributes {
            write!(f, " {}=\"{}\"", attribute, value)?;
        }

        if self.self_closing() {
            write!(f, "/>")
        }
        else{
            write!(f, ">")?;

            for child in &self.children {
                write!(f, "{}", child)?;
            }

            write!(f, "</{}>", self.name)
        }
    }
}