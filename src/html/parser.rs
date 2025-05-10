use super::dom::Node;
use super::tokenizer::{Token, Tokenizer};
use crate::html::dom::NodeType;

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
        if let Token::Doctype = self.current_token {
            self.consume_token();
        }

        let mut root = None;

        while self.current_token != Token::EOF {
            if let Some(node) = self.parse_node() {
                if root.is_none() {
                    root = Some(node);
                } else {
                    panic!("Multiple root nodes found");
                }
            }
        }

        root.expect("No root node found")
    }

    fn parse_node(&mut self) -> Option<Node> {
        match &self.current_token {
            Token::StartTag(name, attrs) => {
                let name = name.clone();
                let attrs = attrs.clone();
                self.consume_token();
                Some(self.parse_element(name, attrs))
            }
            Token::SelfClosingTag(name, attrs) => {
                let name = name.clone();
                let attrs = attrs.clone();
                self.consume_token();
                Some(Node::elem(name, attrs, vec![], true)) // true for self_closing
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

        Node::elem(tag_name, attributes, children, false)
    }

    fn consume_token(&mut self) {
        self.current_token = self.tokenizer.next_token();
    }
}
