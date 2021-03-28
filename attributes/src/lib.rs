use std::collections::HashMap;

pub type AttributeName = String;
pub type AttributeValue = String;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Attributes(HashMap<AttributeName, AttributeValue>);

impl Attributes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn extend<T, K, V>(&mut self, values: T)
    where
        T: AsRef<[(K, V)]>,
        K: Into<String> + Clone,
        V: Into<String> + Clone,
    {
        self.0.extend(
            values
                .as_ref()
                .into_iter()
                .cloned()
                .map(|(name, val)| (name.into(), val.into())),
        );
    }

    pub fn get(&self, name: impl Into<AttributeName>) -> Option<&AttributeValue> {
        self.0.get(&name.into())
    }

    pub fn set<K, V>(&mut self, name: K, value: V)
    where
        K: Into<AttributeName>,
        V: Into<AttributeValue>,
    {
        self.0.insert(name.into(), value.into());
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl PartialEq<HashMap<AttributeName, AttributeValue>> for Attributes {
    fn eq(&self, other: &HashMap<AttributeName, AttributeValue>) -> bool {
        &self.0 == other
    }
}

pub trait AttributesContainer {
    fn get_attr<T>(&self, name: T) -> Option<&str>
    where
        T: Into<String>;

    fn with_attrs<T, K, V>(self, attrs: T) -> Self
    where
        T: AsRef<[(K, V)]>,
        K: Into<String> + Clone,
        V: Into<String> + Clone;
}

#[cfg(test)]
mod tests {

    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_init() {
        let attrs = Attributes::new();
        assert_eq!(attrs.is_empty(), true);
    }

    #[test]
    fn test_extend_slice() {
        let mut attrs = Attributes::new();

        attrs.extend([("hello", "world"), ("goodbye", "me")]);
        assert_eq!(attrs.len(), 2);
        let expected = "world".to_string();
        assert_eq!(attrs.get("hello"), Some(&expected));
        let expected = "me".to_string();
        assert_eq!(attrs.get("goodbye"), Some(&expected));
    }

    #[test]
    fn test_extend_vec() {
        let mut attrs = Attributes::new();

        attrs.extend(vec![
            ("attr1", "val1"),
            ("attr2", "val2"),
            ("attr3", "val3"),
        ]);
        assert_eq!(attrs.len(), 3);
        let expected = "val1".to_string();
        assert_eq!(attrs.get("attr1"), Some(&expected));
        let expected = "val2".to_string();
        assert_eq!(attrs.get("attr2"), Some(&expected));
        let expected = "val3".to_string();
        assert_eq!(attrs.get("attr3"), Some(&expected));
    }

    #[test]
    fn test_multiple_extend() {
        let mut attrs = Attributes::new();

        attrs.extend(vec![("attr1", "val1"), ("attr2", "val2")]);
        assert_eq!(attrs.len(), 2);
        let expected = "val1".to_string();
        assert_eq!(attrs.get("attr1"), Some(&expected));
        let expected = "val2".to_string();
        assert_eq!(attrs.get("attr2"), Some(&expected));

        attrs.extend([("attr3", "val3")]);
        assert_eq!(attrs.len(), 3);

        let expected = "val3".to_string();
        assert_eq!(attrs.get("attr3"), Some(&expected));
    }

    #[test]
    fn test_set() {
        let mut attrs = Attributes::new();

        attrs.set("Hello", "World");
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs.get("Hello"), Some(&"World".to_string()));

        attrs.set("AnotherAttr", "Val");
        assert_eq!(attrs.len(), 2);
        assert_eq!(attrs.get("AnotherAttr"), Some(&"Val".to_string()));

        attrs.set("Hello", "Val1");
        assert_eq!(attrs.len(), 2);
        assert_eq!(attrs.get("Hello"), Some(&"Val1".to_string()));
    }

    #[test]
    fn test_eq() {
        let mut attrs = Attributes::new();

        attrs.extend([("attr1", "val1"), ("attr2", "val2")]);
        let expected = hashmap! {
            "attr1".to_string() => "val1".to_string(),
            "attr2".to_string() => "val2".to_string(),
        };

        assert_eq!(attrs, expected);
    }

    struct SomeStruct {
        pub attrs: Attributes,
    }

    impl SomeStruct {
        pub fn new() -> Self {
            SomeStruct {
                attrs: Attributes::new(),
            }
        }
    }

    impl AttributesContainer for SomeStruct {
        fn get_attr<T>(&self, name: T) -> Option<&str>
        where
            T: Into<String>,
        {
            self.attrs.get(name).map(|v| v.as_str())
        }

        fn with_attrs<T, K, V>(mut self, attrs: T) -> Self
        where
            T: AsRef<[(K, V)]>,
            K: Into<String> + Clone,
            V: Into<String> + Clone,
        {
            self.attrs.extend(attrs);
            self
        }
    }

    #[test]
    fn test_container() {
        let container = SomeStruct::new();

        assert_eq!(container.get_attr("attr1"), None);
    }

    #[test]
    fn test_container_extend() {
        let container = SomeStruct::new().with_attrs([("attr1", "val1"), ("attr2", "val2")]);

        assert_eq!(container.attrs.len(), 2);
        assert_eq!(container.get_attr("attr1"), Some("val1"));
        assert_eq!(container.get_attr("attr2"), Some("val2"));
    }
}
