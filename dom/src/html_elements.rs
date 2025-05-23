use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::rc::{Rc, Weak};

/// Some html tags are self-closing and do not absolutely need an ending Slash
/// This is the case with `<br>`, for example (which can also be written `<br/>`)
/// These elements cannot have children.
pub const __SELF_CLOSED: &[&str] = &[
    "are", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr", "command", "keygen", "menuitem", "frame", "!doctype",
];

#[derive(Debug, Clone)]
/// Either Text Contents or a [`HTMLElement`](HTMLNode)
pub enum HTMLEnum {
    Text(String),
    Node(HTMLNodeRef),
    Comment(String),
}

/// A Weak reference to a HTMLElement.
/// Used by children to reference their parents, without actually owning them
pub type HTMLNodeWeakRef = Weak<RefCell<HTMLNode>>;

/// A Strong reference to a HTMLElement.
/// Used by parents to reference their children.
pub type HTMLNodeRef = Rc<RefCell<HTMLNode>>;

#[derive(Debug, Clone)]
/// A HTML tag.
pub struct HTMLNode {
    ///The name of this tag.
    /// Ex: for a ```<div></div>```, `name` = "div"
    pub name: String,

    ///This tag's attributes
    pub(crate) attributes: HashMap<String, Option<String>>,

    ///This tag's contents, whether they be tags or text.
    pub(crate) children: Vec<HTMLEnum>,

    ///This tag's parent (as a weak reference)
    pub(crate) parent: Option<HTMLNodeWeakRef>,

    //A weak reference to the self, to pass around
    weak_self: HTMLNodeWeakRef,
}

impl HTMLNode {
    //Create a reference to a new element
    pub fn new() -> HTMLNodeRef {
        let elem = Rc::new(RefCell::new(HTMLNode {
            name: String::default(),
            attributes: HashMap::default(),
            children: vec![],
            parent: None,
            weak_self: Default::default(),
        }));

        elem.borrow_mut().weak_self = Rc::downgrade(&elem);
        elem
    }

    /// Gives a reference to this element
    pub fn reference(&self) -> Option<HTMLNodeRef> {
        self.weak_self.upgrade()
    }

    /// Gives a weak reference to this element
    pub fn weak_reference(&self) -> HTMLNodeWeakRef {
        self.weak_self.clone()
    }

    /// Creates a copy of this node, with the same name and attributes.
    /// Children and parents are not copied.
    pub fn duplicate(&self) -> HTMLNodeRef {
        let dup = Self::new();

        let mut dup_borrow = dup.borrow_mut();
        dup_borrow.name = self.name.clone();
        dup_borrow.attributes = self.attributes.clone();
        drop(dup_borrow);
        //Cloning the parent reference would make no sense, as the parent wouldn't be pointing to a newly created element

        dup
    }

    /// Creates a copy of this node, with the same name, attributes, and (copy of its) children
    /// Its parent is not copied, and as such, the duplicate is an orphan
    pub fn duplicate_family(&self) -> HTMLNodeRef {
        let replicant = self.duplicate();
        let mut replicant_borrow = replicant.borrow_mut();

        for child in &self.children {
            match child {
                HTMLEnum::Node(noderef) => {
                    replicant_borrow.add_child(noderef.borrow().duplicate_family());
                }
                other => {
                    replicant_borrow.add_children(vec![other.clone()]);
                }
            }
        }

        drop(replicant_borrow);
        replicant
    }

    /// Returns a strong reference to this element's parent
    pub fn parent(&self) -> Option<HTMLNodeRef> {
        match &self.parent {
            None => None,
            Some(parent) => Weak::upgrade(&parent),
        }
    }

    //Edit the thing
    pub fn attribute(&mut self, attribute: String, value: Option<String>) -> &HTMLNode {
        //Attributes are only meant to be added / modified, not removed
        self.attributes.insert(attribute, value);
        self
    }
    pub fn add_child(&mut self, child: HTMLNodeRef) -> &mut Self {
        child.borrow_mut().orphanize();
        child.borrow_mut().parent = Some(self.weak_self.clone());
        self.children.push(HTMLEnum::Node(child));
        self
    }
    pub fn add_children(&mut self, children: Vec<HTMLEnum>) -> &mut Self {
        for child in children {
            self.children.push(child);
        }
        self
    }
    pub fn add_text<'x>(&'x mut self, text: String) -> &'x Self {
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
                    HTMLEnum::Node(reference) => {
                        Weak::ptr_eq(&Rc::downgrade(reference), &self.weak_self)
                    }
                    _ => false,
                }) {
                    None => {
                        panic!("A child has been disowned. This should not happen");
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

    /// We return an option to an option because
    /// If the attribute doesnt appear, the result is None
    /// If the attribute appears with no value, the result is Some(None)
    /// If the attribute appears with a value, the result is Some(value)
    pub fn get_attribute(&self, name: &str) -> Option<Option<String>> {
        match self
            .attributes
            .iter()
            .find(|(key, _attribute)| key.to_string() == name.to_string())
        {
            Some((_key, val)) => return Some(val.clone()),
            None => {
                return None;
            }
        }
    }
    pub fn children(&self) -> &Vec<HTMLEnum> {
        &self.children
    }
    pub fn self_closing(&self) -> bool {
        __SELF_CLOSED.iter().find(|&&x| x == self.name).is_some()
    }

    // Returns the chain of parents of this node, starting from the closest
    pub fn parent_chain(&self) -> Vec<HTMLNodeRef> {
        let mut result = vec![];

        let mut x: HTMLNodeRef;

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
    pub fn rec_html_children(&self) -> Vec<HTMLNodeRef> {
        let mut result = vec![];

        for child in &self.children {
            match child {
                HTMLEnum::Node(html) => {
                    result.push(html.clone());
                    result.append(&mut html.borrow().rec_html_children());
                }
                _ => {}
            }
        }

        return result;
    }
}

pub trait PrettyPrintable {
    fn pretty_fmt(&self) -> String {
        self.pretty_fmt_rec(0)
    }
    fn pretty_fmt_rec(&self, depth: usize) -> String;
}

impl PrettyPrintable for HTMLEnum {
    fn pretty_fmt_rec(&self, depth: usize) -> String {
        let mut buf = String::new();

        match &self {
            HTMLEnum::Text(t) => {
                buf += &format!(
                    "{}",
                    t.lines()
                        .map(|x| format!("{}{}\n", "\t".repeat(depth), &x))
                        .collect::<Vec<String>>()
                        .join("\n")
                );
            }
            HTMLEnum::Node(elem) => {
                buf += &elem.borrow().pretty_fmt_rec(depth);
            }
            HTMLEnum::Comment(t) => {
                buf += &"\t".repeat(depth);
                buf += &format!("<!--{}-->\n", t);
            }
        }
        buf
    }
}

impl PrettyPrintable for HTMLNode {
    fn pretty_fmt_rec(&self, depth: usize) -> String {
        let mut buf = String::new();

        buf += &format!("{}", "\t".repeat(depth));
        buf += &format!("<{}", self.name);

        for (attr, val) in &self.attributes {
            buf += &format!("{}={}", attr, val.clone().unwrap_or(String::from(r#""""#)))
        }

        if self.self_closing() {
            buf += ">";
        } else {
            buf += ">";

            for child in &self.children {
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

impl Display for HTMLEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HTMLEnum::Text(str) => {
                write!(f, "{}", str)
            }
            HTMLEnum::Node(elem) => {
                write!(f, "{}", elem.borrow().pretty_fmt())
            }
            HTMLEnum::Comment(str) => {
                write!(f, "<!--{}-->\n", str)
            }
        }
    }
}
impl Display for HTMLNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.name)?;

        for (attribute, value) in &self.attributes {
            write!(
                f,
                " {}=\"{}\"",
                attribute,
                value.clone().unwrap_or(r#""""#.to_string())
            )?;
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
