use attributes::{Attributes, AttributesContainer};
use attributes_derive::AttributesContainer;

#[derive(Clone, Debug, PartialEq, AttributesContainer)]
pub struct Edge {
    pub a: String,
    pub b: String,
    #[attributes]
    pub attrs: Attributes,
}

impl Edge {
    pub fn new<A, B>(a: A, b: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            a: a.into(),
            b: b.into(),
            attrs: Attributes::new(),
        }
    }
}
