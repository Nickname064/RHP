use crate::rtml::html_elements::HTMLElement;

///A representation of standard dom queries
pub struct HQuery<'a>{

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
    Or(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>),
    DirectChild(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>),
    IndirectChild(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>)
}

impl<'a> HQuery<'_>{

    pub fn from_str(source : &'a str) -> HQuery{

        // NAME#ID.CLASS[attr=value]

        //Special chars : #, ., [
        let special_chars = &['#', '.', '['];
        let bytes = source.as_bytes();

        let mut result = HQuery{
            name: None,
            class: vec![],
            id: None,
            attributes: vec![],
        };


        panic!("Implement Query parsing !");
    }

    pub fn matches(&self, html_node : &HTMLElement<'a>) -> bool{

        match self.name{
            Some(n) if n != html_node.name => { return false; }
            _ => {}
        }

        let split_class  : Vec<&str> = html_node
            .get_attribute("class")
            .unwrap_or("")
            .split(" ")
            .collect();

        for class in &self.class{
            if !split_class.contains(&class){ return false; }
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

            HCombinedQuery::Or(Box::new(
                (HCombinedQuery::from_str(sliceA),
                HCombinedQuery::from_str(sliceB))
            ))

        } else if let Some(direct_separator) = source.find(">"){
            // SELECTION > SELECTION

            let sliceA = &source[0 .. direct_separator].trim_end();
            let sliceB = &source[direct_separator + 1 ..].trim_start();
            HCombinedQuery::DirectChild(Box::new((
                HCombinedQuery::from_str(sliceA),
                HCombinedQuery::from_str(sliceB)
            )))

        } else if let Some(indirect_separator) = source.find(" "){
            //SELECTION SELECTION

            let sliceA = &source[0 .. indirect_separator].trim_end();
            let sliceB = &source[indirect_separator + 1 ..].trim_start();
            HCombinedQuery::IndirectChild( Box::new((
                HCombinedQuery::from_str(sliceA),
                HCombinedQuery::from_str(sliceB)
            )))

        } else {
            HCombinedQuery::Simple(
                HQuery::from_str(source)
            )
        }
    }

    pub fn get_matches(&self, html_node : &HTMLElement<'a>) -> Vec<HTMLElement<'a>>{

        todo!("Implement this once html nodes have parents");

        vec![]
    }

    pub fn matches(&self, html_node : &HTMLElement<'a>) -> bool{
        match self{
            HCombinedQuery::Simple(Query) => { Query.matches(html_node) }
            HCombinedQuery::Or(tuple) => {
                let (ref a, ref b) = **tuple;
                a.matches(html_node) || b.matches(html_node)
            }
            HCombinedQuery::DirectChild(tuple) => { panic!("TODO : Implement Direct children in combined queries !") }
            HCombinedQuery::IndirectChild(_) => { panic!("TODO : Implement Indirect children in combined queries")}
        }
    }
}

