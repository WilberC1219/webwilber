use std::collections::{HashMap, HashSet};
pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

#[derive(Debug)]
pub enum NodeType { 
    Element(ElementData),
    Text(String),
}

// constuctor functions

pub fn text(data: String) -> Node {
    Node { 
        children: vec![], 
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