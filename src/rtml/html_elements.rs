use std::cell::RefCell;
use std::fmt::{Display, format, Formatter};
use std::rc::{Rc, Weak};

use crate::rtml::reparse::SELF_CLOSABLE_TAGS;

#[derive(Debug, Clone)]
/// Either Text Contents or a [`HTMLElement`](HTMLElement)
pub enum HTMLEnum<'a> {
    Text(&'a str),
    Element(HTMLElementReference<'a>),
    Comment(&'a str),
}

/// A Weak reference to a HTMLElement.
/// Used by children to reference their parents, without actually owning them
pub type HTMLElementWeakReference<'a> = Weak<RefCell<HTMLElement<'a>>>;

/// A Strong reference to a HTMLElement.
/// Used by parents to reference their children.
pub type HTMLElementReference<'a> = Rc<RefCell<HTMLElement<'a>>>;

#[derive(Debug, Clone)]
/// A HTML tag.
pub struct HTMLElement<'a> {
    ///The name of this tag.
    /// Ex: for a ```<div></div>```, `name` = "div"
    pub(crate) name: &'a str,

    ///Hermes Feature
    ///Empty for regular HTML Elements
    ///
    /// Used by custom HTML tags to pass arguments to the underlying functions
    /// No need to read the arguments by hand, as elements with tags must be compiled to regular elements before they can be served
    pub(crate) args: Vec<(&'a str, &'a str)>,

    ///This tag's attributes
    pub(crate) attributes: Vec<(&'a str, &'a str)>,

    ///This tag's contents, whether they be tags or text.
    pub(crate) children: Vec<HTMLEnum<'a>>,

    ///This tag's parent (as a weak reference)
    pub(crate) parent: Option<HTMLElementWeakReference<'a>>,

    //A weak reference to the self, to pass around
    weak_self: HTMLElementWeakReference<'a>,
}

impl<'a> HTMLElement<'a> {
    //Create a reference to a new element
    pub fn new() -> HTMLElementReference<'a> {
        let elem = Rc::new(RefCell::new(HTMLElement {
            name: "",
            args: vec![],
            attributes: vec![],
            children: vec![],
            parent: None,
            weak_self: Default::default(),
        }));

        elem.borrow_mut().weak_self = Rc::downgrade(&elem);
        elem
    }

    /// Gives a reference to this element
    pub fn reference(&self) -> Option<HTMLElementReference<'a>> {
        self.weak_self.upgrade()
    }

    /// Gives a weak reference to this element
    pub fn weak_reference(&self) -> HTMLElementWeakReference<'a> {
        self.weak_self.clone()
    }

    pub fn duplicate(&self) -> HTMLElementReference<'a>{
        let dup = Self::new();

        let mut dup_borrow = dup.borrow_mut();
        dup_borrow.name = self.name;
        dup_borrow.args = self.args.clone();
        dup_borrow.attributes = self.attributes.clone();
        drop(dup_borrow);
        //Cloning the parent reference would make no sense, as the parent wouldn't be pointing to a newly created element

        dup
    }

    /// Returns a strong reference to this element's parent
    pub fn parent(&self) -> Option<HTMLElementReference<'a>> {
        match &self.parent {
            None => None,
            Some(parent) => Weak::upgrade(&parent),
        }
    }

    //Edit the thing
    pub fn attribute(&mut self, attribute: &'a str, value: &'a str) -> &HTMLElement<'a> {
        //Attributes are only meant to be added / modified, not removed

        if let Some(index) = self
            .attributes
            .iter()
            .position(|(name, _)| *name == attribute)
        {
            self.attributes[index] = (attribute, value);
        } else {
            //TODO : Sort insertion
            self.attributes.push((attribute, value));
        }
        self
    }
    pub fn argument(&'a mut self, arg_name: &'a str, value: &'a str) -> &HTMLElement<'a> {
        //Attributes are only meant to be added, not removed

        if let Some(index) = self.args.iter().position(|(name, _)| *name == arg_name) {
            self.args[index] = (arg_name, value);
        } else {
            //TODO : Sort insertion
            self.attributes.push((arg_name, value));
        }
        self
    }
    pub fn add_child(&mut self, child: HTMLElementReference<'a>) -> &mut Self {
        child.borrow_mut().orphanize();
        child.borrow_mut().parent = Some(self.weak_self.clone());
        self.children.push(HTMLEnum::Element(child));
        self
    }
    pub fn add_children(&mut self, children: Vec<HTMLEnum<'a>>) -> &mut Self {
        for child in children {
            self.children.push(child);
        }
        self
    }
    pub fn add_text<'x>(&'x mut self, text: &'a str) -> &'x Self {
        self.children.push(HTMLEnum::Text(text));
        self
    }

    ///Disconnect this node from its parent
    pub fn orphanize(&mut self) -> &mut Self {
        match self.parent() {
            None => self, //No parent, the job is done
            Some(parent) => {
                let mut parent_borrow = parent.borrow_mut();

                match parent_borrow.children.iter().position(|x| match x {
                    HTMLEnum::Element(reference) => {
                        Weak::ptr_eq(&Rc::downgrade(reference), &self.weak_self)
                    }
                    _ => false,
                }) {
                    None => {
                        panic!("A child doesn't appear in its parent children");
                    }
                    Some(index) => {
                        parent_borrow.children.remove(index);
                        self.parent = None;
                        return self;
                    }
                }
            }
        }
    }

    //Getter methods
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        match self
            .attributes
            .iter()
            .find(|(key, _attribute)| key.to_string() == name.to_string())
        {
            Some((_key, val)) => return Some(val),
            None => {
                return None;
            }
        }
    }
    pub fn children(&self) -> &Vec<HTMLEnum<'a>> {
        &self.children
    }
    pub fn self_closing(&self) -> bool {
        SELF_CLOSABLE_TAGS
            .iter()
            .find(|&&x| x == self.name)
            .is_some()
    }

    // Returns the chain of parents of this node, starting from the closest
    pub fn parent_chain(&self) -> Vec<HTMLElementReference<'a>> {
        let mut result = vec![];

        let mut x: HTMLElementReference;

        if let Some(parent) = self.parent() {
            x = parent.clone();
            result.push(parent);
        } else {
            return result;
        }

        loop {
            let par = x.borrow().parent();

            if let Some(parent) = par {
                x = parent.clone();
                result.push(parent);
            } else {
                break;
            }
        }

        result
    }

    // Returns the chain of HTMLElement children of this node, in a breadth-first order
    // Useful for quickly looking through nodes
    pub fn rec_html_children(&self) -> Vec<HTMLElementReference<'a>> {
        let mut result = vec![];

        for child in &self.children {
            match child {
                HTMLEnum::Element(html) => {
                    result.push(html.clone());
                    result.append(&mut html.borrow().rec_html_children());
                }
                _ => {}
            }
        }

        return result;
    }
}

impl HTMLEnum<'_> {
    fn fmt(&self, f: &mut Formatter<'_>, _rec_level: usize) -> std::fmt::Result {
        match self {
            HTMLEnum::Text(str) => {
                write!(f, "{}", str)
            }
            HTMLEnum::Element(elem) => {
                write!(f, "{}", elem.borrow())
            }
            HTMLEnum::Comment(str) => {
                write!(f, "<!--{}-->", str)
            }
        }
    }
}


pub trait PrettyPrintable{
    fn pretty_fmt(&self) -> String { self.pretty_fmt_rec(0) }
    fn pretty_fmt_rec(&self, depth : usize) -> String;
}

impl PrettyPrintable for HTMLEnum<'_>{
    fn pretty_fmt_rec(&self, depth : usize) -> String {
        let mut buf = String::new();
        
        match &self{
            HTMLEnum::Text(t) => {
                buf += &format!("{}{}", "\t".repeat(depth), t);
            }
            HTMLEnum::Element(elem) => {
                buf += &elem.borrow().pretty_fmt_rec(depth);
            }
            HTMLEnum::Comment(t) => {
                buf += &"\t".repeat(depth);
                buf += &format!("<!--{}-->", t);
            }
        }
        buf
    }
}
impl PrettyPrintable for HTMLElement<'_>{
    fn pretty_fmt_rec(&self, depth : usize) -> String {
        let mut buf = String::new();

        buf += &format!("{}", "\t".repeat(depth));
        buf += &format!("<{}", self.name);

        for (attr, val) in &self.attributes{
            if val.len() > 0 {
                buf += &format!(" {}={}", attr, val);
            } else{
                buf += &format!(" {}", attr);
            }
        }

        if self.self_closing(){
            buf += ">";
        } else{
            buf += ">";

            for child in &self.children{
                buf += "\n";
                buf += &child.pretty_fmt_rec(depth + 1);
            }

            if !self.children.is_empty() {
                buf += "\n";
                buf += &"\t".repeat(depth);
            }

            buf += &format!("</{}>", self.name);
        }
        
        buf
    }
}

impl Display for HTMLEnum<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HTMLEnum::Text(str) => {
                write!(f, "{}", str)
            }
            HTMLEnum::Element(elem) => {
                write!(f, "{}", elem.borrow())
            }
            HTMLEnum::Comment(str) => {
                write!(f, "<!--{}-->\n", str)
            }
        }
    }
}
impl Display for HTMLElement<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.name)?;

        for (attribute, value) in &self.attributes {
            write!(f, " {}=\"{}\"", attribute, value)?;
        }

        if self.self_closing() {
            write!(f, "/>")
        } else {
            write!(f, ">")?;

            for child in &self.children {
                write!(f, "{}", child)?;
            }

            write!(f, "</{}>", self.name)
        }
    }
}
