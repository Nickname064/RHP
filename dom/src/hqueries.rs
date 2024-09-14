use crate::hqueries;
use crate::hqueries::HQueryErr::DuplicateId;
use crate::html_elements::HTMLNode;

///A representation of standard dom queries
#[derive(Debug)]
pub struct HQuery<'a> {
    ///Tag name of the object
    name: Option<&'a str>,

    ///Classes the object must have
    class: Vec<&'a str>,

    /// IDentifier of the object
    id: Option<&'a str>,

    /// Attribute-value pairs
    attributes: Vec<(&'a str, Option<&'a str>)>,
}

#[derive(Debug)]
pub enum HCombinedQuery<'a> { //TODO: Rework representation
    Simple(HQuery<'a>),
    Or(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>),
    DirectChild(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>),
    IndirectChild(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>),
}

#[derive(Debug)]
pub enum HQueryErr {
    DuplicateId,
}

/// A simplified DOM query, implementing classes, tag names, identifiers, and attribute-value pairs
impl<'a> HQuery<'_> {
    pub fn from_str(mut source: &'a str) -> Result<HQuery, HQueryErr> {
        // NAME#ID.CLASS[attr=value]

        //Special chars : #, ., [
        let _special_chars = &['#', '.', '['];
        let _bytes = source.as_bytes();

        let mut _result = HQuery {
            name: None,
            class: vec![],
            id: None,
            attributes: vec![],
        };

        //Find name
        if let Some(index) = source.find(|x| _special_chars.contains(&x)) {
            if index != 0 {
                //Extract name
                _result.name = Some(&source[..index]);
                source = &source[index..];
            }
        } else {
            return Ok(_result);
        }

        loop {
            let letter = source.chars().nth(0);
            if source.len() > 0 {
                source = &source[1..]
            };

            //While source.len() > 0 !TRUST
            //Extract other fields
            match letter {
                //Comsumes 0-th
                None => {
                    break;
                }
                Some('.') =>
                /*CLASS*/
                {
                    let index = source
                        .chars()
                        .position(|x: char| _special_chars.contains(&x))
                        .unwrap_or(source.len());
                    _result.class.push(&source[..index]);
                    source = &source[index..];
                }
                Some('#') =>
                /*IDENTIFIER*/
                {
                    let index = source
                        .chars()
                        .position(|x: char| _special_chars.contains(&x))
                        .unwrap_or(source.len());

                    if _result.id.is_some() {
                        return Err(DuplicateId);
                    }

                    _result.id = Some(&source[..index]);
                    source = &source[index..];
                }
                Some('[') =>
                /*ATTR-VALUE*/
                {
                    todo!("Implement attribute-value queries !")
                }
                Some(unimplemented) => {
                    panic!(
                        "Unimplemented yet detected query symbol ! [{}]",
                        unimplemented
                    )
                }
            }
        }

        Ok(_result)
    }

    pub fn matches(&self, html_node: &HTMLNode<'a>) -> bool {
        match self.name {
            Some(n) if n != html_node.name => {
                return false;
            }
            _ => {}
        }

        let split_class: Vec<&str> = html_node
            .get_attribute("class")
            .unwrap_or(Some(""))
            .unwrap_or("")
            .split(" ")
            .collect();

        for class in &self.class {
            if !split_class.contains(&class) {
                return false;
            }
        }

        match self.id {
            Some(id) => { return html_node.get_attribute("id") == Some(Some(id)); }
            None => {}
        }

        for (attribute, maybe_value) in &self.attributes{
            let nodeval =  html_node.get_attribute(attribute);

            match maybe_value{
                None => { if nodeval.is_none() { return false; } }
                Some(value) => { if nodeval.is_none() || nodeval.unwrap() != Some(value) { return false; } }
            }
        }

        true
    }
}


