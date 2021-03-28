use attributes::{Attributes, AttributesContainer};
use attributes_derive::AttributesContainer;

#[derive(Clone, Debug, PartialEq, AttributesContainer)]
pub struct Node {
    pub name: String,
    #[attributes]
    pub attrs: Attributes,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            attrs: Attributes::new(),
        }
    }
}
