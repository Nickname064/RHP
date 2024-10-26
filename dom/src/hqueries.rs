use crate::hqueries::HQueryErr::DuplicateId;
use crate::html_elements::HTMLNode;

///A representation of standard dom queries
#[derive(Debug)]
pub struct HQuery {
    ///Tag name of the object
    name: Option<String>,

    ///Classes the object must have
    class: Vec<String>,

    /// IDentifier of the object
    id: Option<String>,

    /// Attribute-value pairs
    attributes: Vec<(String, Option<String>)>,
}

#[derive(Debug)]
pub enum HCombinedQuery {
    //TODO: Rework representation
    Simple(HQuery),
    Or(Box<(HCombinedQuery, HCombinedQuery)>),
    DirectChild(Box<(HCombinedQuery, HCombinedQuery)>),
    IndirectChild(Box<(HCombinedQuery, HCombinedQuery)>),
}

#[derive(Debug)]
pub enum HQueryErr {
    DuplicateId,
}

/// A simplified DOM query, implementing classes, tag names, identifiers, and attribute-value pairs
impl HQuery {
    pub fn from_str(mut source: String) -> Result<HQuery, HQueryErr> {
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
                _result.name = Some(String::from(&source[..index]));
                source = String::from(&source[index..]);
            }
        } else {
            return Ok(_result);
        }

        loop {
            let letter = source.chars().nth(0);
            if source.len() > 0 {
                source = String::from(&source[1..]);
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
                    _result.class.push(String::from(&source[..index]));
                    source = String::from(&source[index..]);
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

                    _result.id = Some(String::from(&source[..index]));
                    source = String::from(&source[index..]);
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

    pub fn matches(&self, html_node: &HTMLNode) -> bool {
        todo!("Matching")
    }
}
