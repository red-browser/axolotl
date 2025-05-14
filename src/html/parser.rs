use super::dom::{self, Node, NodeType};
use super::tokenizer::{Token, Tokenizer};

pub struct Parser {
    tokenizer: Tokenizer,
    current_token: Token,
}

impl Parser {
    pub fn new(input: String) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let current_token = tokenizer.next_token();
        Parser {
            tokenizer,
            current_token,
        }
    }

    pub fn parse(&mut self) -> Node {
        let mut children = vec![];
        let mut has_doctype = false;
        let mut has_html = false;

        // Parse doctype if present
        if let Token::Doctype = self.current_token {
            children.push(Node::doctype());
            has_doctype = true;
            self.consume_token();
        }

        while self.current_token != Token::EOF {
            if let Some(node) = self.parse_node() {
                if let NodeType::Element(ref elem) = node.node_type {
                    if elem.tag_name == "html" {
                        has_html = true;
                    }
                }
                children.push(node);
            }
        }

        if !has_html {
            let mut html_children = vec![];
            let mut head = None;
            let mut body_children = vec![];

            for child in children {
                if let NodeType::Element(ref elem) = child.node_type {
                    if elem.tag_name == "head" {
                        head = Some(child);
                        continue;
                    }
                }
                body_children.push(child);
            }

            let head =
                head.unwrap_or_else(|| Node::elem("head".to_string(), vec![], vec![], false));

            let body = Node::elem("body".to_string(), vec![], body_children, false);

            html_children.push(head);
            html_children.push(body);

            children = vec![Node::elem("html".to_string(), vec![], html_children, false)];
        }

        Node::new(NodeType::Document, children)
    }

    fn parse_node(&mut self) -> Option<Node> {
        match &self.current_token {
            Token::StartTag(name, attrs) => {
                let name = name.clone();
                let attrs = attrs.clone();
                self.consume_token();

                if dom::is_void_element(&name) {
                    Some(Node::elem(name, attrs, vec![], true))
                } else {
                    Some(self.parse_element(name, attrs))
                }
            }
            Token::SelfClosingTag(name, attrs) => {
                let name = name.clone();
                let attrs = attrs.clone();
                self.consume_token();
                Some(Node::elem(name, attrs, vec![], true))
            }
            Token::Text(text) => {
                let text = text.clone();
                self.consume_token();
                Some(Node::text(text))
            }
            Token::Comment(text) => {
                let text = text.clone();
                self.consume_token();
                Some(Node::comment(text))
            }
            Token::Doctype => {
                self.consume_token();
                Some(Node::doctype())
            }
            Token::EndTag(_) => {
                self.consume_token();
                None
            }
            Token::EOF => None,
        }
    }

    fn parse_element(&mut self, tag_name: String, attributes: Vec<(String, String)>) -> Node {
        let mut children = vec![];
        let mut stack = vec![tag_name.clone()];

        while self.current_token != Token::EOF {
            match &self.current_token {
                Token::EndTag(end_tag) if end_tag == &tag_name => {
                    self.consume_token();
                    break;
                }
                Token::EndTag(unexpected_end) => {
                    if stack.last() == Some(unexpected_end) {
                        stack.pop();
                        self.consume_token();
                    } else {
                        self.consume_token();
                    }
                }
                _ => {
                    if let Some(child) = self.parse_node() {
                        if let NodeType::Element(ref elem) = child.node_type {
                            if !dom::is_void_element(&elem.tag_name) {
                                stack.push(elem.tag_name.clone());
                            }
                        }
                        children.push(child);
                    }
                }
            }
        }

        Node::elem(tag_name, attributes, children, false)
    }

    fn consume_token(&mut self) {
        self.current_token = self.tokenizer.next_token();
    }
}
