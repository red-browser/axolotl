use super::dom::Node;
use super::tokenizer::{Token, Tokenizer};
use crate::html::dom;

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
        let mut root_children = vec![];

        // Skip doctype if present
        if let Token::Doctype = self.current_token {
            self.consume_token();
        }

        // Parse all nodes (HTML may have multiple root nodes in fragments)
        while self.current_token != Token::EOF {
            if let Some(node) = self.parse_node() {
                root_children.push(node);
            }
        }

        // Create a root node containing all top-level nodes
        Node::elem("html".to_string(), vec![], root_children)
    }

    fn parse_node(&mut self) -> Option<Node> {
        match &self.current_token {
            Token::StartTag(name, attrs) => {
                let name = name.clone();
                let attrs = attrs.clone();
                self.consume_token();

                if dom::is_void_element(&name) {
                    Some(Node::elem(name, attrs, vec![]))
                } else {
                    Some(self.parse_element(name, attrs))
                }
            }
            Token::SelfClosingTag(name, attrs) => {
                let name = name.clone();
                let attrs = attrs.clone();
                self.consume_token();
                Some(Node::elem(name, attrs, vec![]))
            }
            Token::Text(text) => {
                let text = text.clone();
                self.consume_token();
                if text.trim().is_empty() {
                    None
                } else {
                    Some(Node::text(text))
                }
            }
            Token::Comment(_) | Token::Doctype => {
                self.consume_token();
                None
            }
            Token::EndTag(_) => {
                // Handle mismatched end tags more gracefully
                self.consume_token();
                None
            }
            Token::EOF => None,
        }
    }

    fn parse_element(&mut self, tag_name: String, attributes: Vec<(String, String)>) -> Node {
        let mut children = vec![];

        while self.current_token != Token::EndTag(tag_name.clone())
            && self.current_token != Token::EOF
        {
            if let Some(child) = self.parse_node() {
                children.push(child);
            }
        }

        if let Token::EndTag(_) = self.current_token {
            self.consume_token();
        }

        Node::elem(tag_name, attributes, children)
    }

    fn consume_token(&mut self) {
        self.current_token = self.tokenizer.next_token();
    }
}
