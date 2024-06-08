use std::collections::{HashMap, HashSet};
use crate::rtml::html_elements::HTMLElement;

/*
///A representation of standard dom queries
struct HQuery<'a>{

    ///Tag name of the object
    name : Option<&'a str>,

    ///Classes the object must have
    class : Vec<&'a str>,

    /// IDentifier of the object
    id : Option<&'a str>,

    /// Attribute-value pairs
    attributes : Vec<(&'a str, &'a str)>,
}

enum HCombinedQuery<'a>{
    Simple(HQuery<'a>),
    Or(HCombinedQuery<'a>, HCombinedQuery<'a>),
    DirectChild(HCombinedQuery<'a>, HCombinedQuery<'a>),
    IndirectChild(HCombinedQuery<'a>, HCombinedQuery<'a>)
}

impl<'a> HQuery<'_>{

    pub fn from_str(source : &'a str) -> HQuery{
        panic!("Implement Query parsing !");

        // NAME#ID.CLASS[attr=value]


    }

    pub fn matches(&self, html_node : &HTMLElement<'a>) -> bool{

        match self.name{
            Some(n) if n != html_node.name => { return false; }
            _ => {}
        }

        let split_class = html_node
            .get_attribute("class")
            .unwrap_or("")
            .split(" ");

        for class in self.class{
            if !split_class.contains(class){ return false; }
        }

        match self.id{
            Some(id) => { html_node.get_attribute("id") == Some(id) }
            None => { true }
        }
    }
}
impl<'a> HCombinedQuery<'a>{

    pub fn from_str(mut source : &'a str) -> HCombinedQuery<'a>{

        if let Some(or_position) = source.find(","){
            //SELECTION, SELECTION

            let sliceA = &source[0 .. or_position].trim_end();
            let sliceB = &source[or_position + 1 ..].trim_start();

            HCombinedQuery::Or(
                HCombinedQuery::from_str(sliceA),
                HCombinedQuery::from_str(sliceB)
            )

        } else if let Some(direct_separator) = source.find(">"){
            // SELECTION > SELECTION

            let sliceA = &source[0 .. direct_separator].trim_end();
            let sliceB = &source[direct_separator + 1 ..].trim_start();
            HCombinedQuery::DirectChild(
                HCombinedQuery::from_str(sliceA),
                HCombinedQuery::from_str(sliceB)
            )

        } else if let Some(indirect_separator) = source.find(" "){
            //SELECTION SELECTION

            let sliceA = &source[0 .. indirect_separator].trim_end();
            let sliceB = &source[indirect_separator + 1 ..].trim_start();
            HCombinedQuery::IndirectChild(
                HCombinedQuery::from_str(sliceA),
                HCombinedQuery::from_str(sliceB)
            )

        } else {
            HCombinedQuery::Simple(
                HQuery::from_str(source)
            )
        }
    }

    pub fn matches(&self, html_node : &HTMLElement<'a>) -> bool{
        match self{
            HCombinedQuery::Simple(Query) => { Query.matches(html_node) }
            HCombinedQuery::Or(A, B) => { A.matches(html_node) || B.matches(html_node) }
            HCombinedQuery::DirectChild(_, _) => { panic!("TODO : Implement Direct children in combined queries !") }
            HCombinedQuery::IndirectChild(_, _) => { panic!("TODO : Implement Indirect children in combined queries")}
        }
    }
}
*/
