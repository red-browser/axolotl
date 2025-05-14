#[derive(Debug, PartialEq)]
pub enum NodeType {
    Document,
    Doctype,
    Element(ElementData),
    Text(String),
    Comment(String),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(Debug, PartialEq)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub is_self_closing: bool,
}

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Self {
        Node {
            children,
            node_type,
        }
    }

    pub fn text(data: String) -> Self {
        Node::new(NodeType::Text(data), vec![])
    }

    pub fn elem(
        name: String,
        attrs: Vec<(String, String)>,
        children: Vec<Node>,
        is_self_closing: bool,
    ) -> Self {
        Node::new(
            NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
                is_self_closing,
            }),
            children,
        )
    }

    pub fn doctype() -> Self {
        Node::new(NodeType::Doctype, vec![])
    }

    pub fn comment(data: String) -> Self {
        Node::new(NodeType::Comment(data), vec![])
    }
}

// List of void elements from HTML5 spec
pub fn is_void_element(tag_name: &str) -> bool {
    matches!(
        tag_name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}
