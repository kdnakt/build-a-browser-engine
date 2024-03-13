use std::collections::{
    HashMap,
    HashSet,
};

#[derive(Debug, PartialEq)]
pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Text(String),
    Comment(String),
    Element(ElementData),
}

pub type AttrMap = HashMap<String, String>;

#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classList) => classList.split(' ').collect(),
            None => HashSet::new()
        }
    }
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data)
    }
}

pub fn comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data)
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}
