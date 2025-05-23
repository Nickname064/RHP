use crate::html_elements::{HTMLNode, HTMLNodeRef};
use std::collections::HashSet;

/// A representation of standard dom queries, but only on one element
#[derive(Debug)]
pub struct HQuery {
    ///Tag name of the object
    name: Option<String>,

    ///Classes the object must have
    classes: Vec<String>,

    /// IDentifier of the object
    id: Option<String>,

    /// Attribute-value pairs
    attributes: Vec<(String, Option<String>)>,
}

/// A simplified DOM query, implementing classes, tag names, identifiers, and attribute-value pairs
impl HQuery {
    pub fn matches(&self, html_node: &HTMLNode) -> bool {
        // Match name
        if let Some(expected_name) = &self.name {
            if html_node.name() != expected_name {
                return false;
            }
        }

        // Match id
        if let Some(expected_id) = &self.id {
            let id_attr = html_node
                .attributes
                .iter()
                .find(|(attr, _)| attr.to_lowercase() == "id");

            // Expect an ID field to be set
            if id_attr.is_none() {
                return false;
            }

            // Expect the ID to be as specified
            let (_, val) = id_attr.unwrap();
            let actual_id = val.clone().unwrap_or("".to_lowercase());

            if &actual_id != expected_id {
                return false;
            }
        }

        // Match attributes
        for (attribute, value) in &self.attributes {
            let actual_value = html_node.attributes.get(attribute); //Opt<Opt<Str>>

            if actual_value.is_none() {
                return false;
            }

            // A None value means we just want the attribute to be defined
            if value.is_none() {
                continue;
            }

            let actual_value = actual_value.unwrap();

            if actual_value.clone().unwrap_or("".to_string()) != value.clone().unwrap() {
                return false;
            }
        }

        // Match classes
        let defined_classes = html_node.attributes.get("class");
        if defined_classes.is_none() {
            return self.classes.len() == 0;
        }

        let mut classes_table: HashSet<String> = HashSet::new();
        defined_classes
            .unwrap()
            .clone()
            .unwrap_or("".to_string())
            .split(" ")
            .filter(|s| s.len() != 0)
            .for_each(|x| {
                classes_table.insert(x.to_string());
            });

        for class in &self.classes {
            if classes_table.get(class).is_none() {
                return false;
            }
        }

        return true;
    }
}

#[derive(Debug)]
pub enum HChildType {
    Direct,
    Indirect,
}

#[derive(Debug)]
pub struct HCombinedQuery<const N: usize> {
    root: HQuery,
    children: [(HQuery, HChildType); N],
}

impl<const N: usize> HCombinedQuery<N> {}

#[derive(Debug)]
pub enum HQueryErr {
    DuplicateId,
}
