use crate::rtml::html_elements::HTMLElement;

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
    attributes: Vec<(&'a str, &'a str)>,
}

#[derive(Debug)]
pub enum HCombinedQuery<'a> {
    Simple(HQuery<'a>),
    Or(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>),
    DirectChild(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>),
    IndirectChild(Box<(HCombinedQuery<'a>, HCombinedQuery<'a>)>),
}

/// A simplified DOM query, implementing classes, tag names, identifiers, and attribute-value pairs
impl<'a> HQuery<'_> {
    pub fn from_str(mut source: &'a str) -> HQuery {
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
            return _result;
        }

        loop {
            let letter = source.chars().nth(0);
            if source.len() > 0 { source = &source[1..] };

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
                        .position(|x : char| _special_chars.contains(&x))
                        .unwrap_or(source.len());
                    _result.class.push(&source[..index]);
                    source = &source[index..];
                }
                Some('#') =>
                /*IDENTIFIER*/
                {
                    let index = source
                        .chars()
                        .position(|x : char| _special_chars.contains(&x))
                        .unwrap_or(source.len());

                    if _result.id.is_some() {
                        todo!("Proper Error Handling, Duplicated ID Field in DOM Query");
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

        _result
    }

    pub fn matches(&self, html_node: &HTMLElement<'a>) -> bool {
        match self.name {
            Some(n) if n != html_node.name => {
                return false;
            }
            _ => {}
        }

        let split_class: Vec<&str> = html_node
            .get_attribute("class")
            .unwrap_or("")
            .split(" ")
            .collect();

        for class in &self.class {
            if !split_class.contains(&class) {
                return false;
            }
        }

        match self.id {
            Some(id) => html_node.get_attribute("id") == Some(id),
            None => true,
        }
    }
}

/// A Dom query, implementing everything but pseudo-elements
impl<'a> HCombinedQuery<'a> {
    pub fn from_str(source: &'a str) -> HCombinedQuery<'a> {
        if let Some(or_position) = source.find(",") {
            //The order doesn't matter
            //SELECTION, SELECTION

            let slice_a = &source[0..or_position].trim_end();
            let slice_b = &source[or_position + 1..].trim_start();

            HCombinedQuery::Or(Box::new((
                HCombinedQuery::from_str(slice_a),
                HCombinedQuery::from_str(slice_b),
            )))
        } else if let Some(direct_separator) = source.rfind(">") {
            // SELECTION > SELECTION

            let slice_a = &source[0..direct_separator].trim_end();
            let slice_b = &source[direct_separator + 1..].trim_start();
            HCombinedQuery::DirectChild(Box::new((
                HCombinedQuery::from_str(slice_a),
                HCombinedQuery::from_str(slice_b),
            )))
        } else if let Some(indirect_separator) = source.rfind(" ") {
            //SELECTION SELECTION

            let slice_a = &source[0..indirect_separator].trim_end();
            let slice_b = &source[indirect_separator + 1..].trim_start();
            HCombinedQuery::IndirectChild(Box::new((
                HCombinedQuery::from_str(slice_a),
                HCombinedQuery::from_str(slice_b),
            )))
        } else {
            HCombinedQuery::Simple(HQuery::from_str(source))
        }
    }

    pub fn matches(&self, html_node: &HTMLElement<'a>) -> bool {
        match self {
            HCombinedQuery::Simple(Query) => Query.matches(html_node), // select
            HCombinedQuery::Or(tuple) => {
                // select1, select2
                let (ref a, ref b) = **tuple;
                a.matches(html_node) || b.matches(html_node)
            }
            HCombinedQuery::DirectChild(tuple) => {
                // select1 > select2
                let (ref a, ref b) = **tuple;
                let hparent = html_node.parent();
                b.matches(html_node) && hparent.is_some() && a.matches(&hparent.unwrap().borrow())
            }
            HCombinedQuery::IndirectChild(tuple) => {
                // select1 select2
                let (ref a, ref b) = **tuple;
                let hparentlist = html_node.parent_chain();
                b.matches(html_node)
                    && hparentlist
                        .iter()
                        .find(|x| a.matches(&x.borrow()))
                        .is_some()
            }
        }
    }
}
