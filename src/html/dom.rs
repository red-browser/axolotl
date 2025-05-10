#[derive(Debug, PartialEq)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
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
    pub self_closing: bool,
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
        self_closing: bool,
    ) -> Self {
        Node::new(
            NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
                self_closing,
            }),
            children,
        )
    }
}
