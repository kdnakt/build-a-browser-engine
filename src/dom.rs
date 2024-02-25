use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    children: Vec<Node>,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data)
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
