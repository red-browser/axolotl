#[derive(Debug, PartialEq)]
pub enum Token {
    Doctype,
    StartTag(String, Vec<(String, String)>), // Added attributes
    EndTag(String),
    Comment(String),
    Text(String),
    EOF,
    SelfClosingTag(String, Vec<(String, String)>),
}
pub struct Tokenizer {
    input: String,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Tokenizer { input, position: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let c = self.input.chars().nth(self.position).unwrap();

        match c {
            '<' => {
                self.position += 1;
                if self.position >= self.input.len() {
                    return Token::Text("<".to_string());
                }

                let next_char = self.input.chars().nth(self.position).unwrap();
                match next_char {
                    '!' => self.parse_doctype_or_comment(),
                    '/' => self.parse_end_tag(),
                    _ => self.parse_start_tag(),
                }
            }
            _ => self.parse_text(),
        }
    }

    fn parse_start_tag(&mut self) -> Token {
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        let is_self_closing = self.position < self.input.len()
            && self.input.chars().nth(self.position).unwrap() == '/';
        if is_self_closing {
            self.position += 1; // Skip '/'
            self.skip_until('>');
            return Token::SelfClosingTag(tag_name, attributes);
        }
        self.skip_until('>');
        Token::StartTag(tag_name, attributes)
    }

    fn parse_attributes(&mut self) -> Vec<(String, String)> {
        let mut attributes = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();
            let c = self.input.chars().nth(self.position).unwrap();

            // Stop if we hit the closing >
            if c == '>' {
                break;
            }

            // Parse attribute name
            let name_start = self.position;
            while self.position < self.input.len() {
                let c = self.input.chars().nth(self.position).unwrap();
                if c.is_whitespace() || c == '=' || c == '>' {
                    break;
                }
                self.position += 1;
            }
            let name = self.input[name_start..self.position].to_ascii_lowercase();

            // Skip whitespace after name
            self.skip_whitespace();

            // Check for attribute value
            if self.position < self.input.len()
                && self.input.chars().nth(self.position).unwrap() == '='
            {
                self.position += 1;
                self.skip_whitespace();

                // Parse attribute value
                let quote = self.input.chars().nth(self.position).unwrap();
                if quote == '"' || quote == '\'' {
                    self.position += 1;
                    let value_start = self.position;

                    while self.position < self.input.len() {
                        if self.input.chars().nth(self.position).unwrap() == quote {
                            break;
                        }
                        self.position += 1;
                    }

                    let value = self.input[value_start..self.position].to_string();
                    self.position += 1; // Skip closing quote

                    attributes.push((name, value));
                } else {
                    // Unquoted attribute value
                    let value_start = self.position;
                    while self.position < self.input.len() {
                        let c = self.input.chars().nth(self.position).unwrap();
                        if c.is_whitespace() || c == '>' {
                            break;
                        }
                        self.position += 1;
                    }
                    let value = self.input[value_start..self.position].to_string();
                    attributes.push((name, value));
                }
            } else {
                // Boolean attribute (no value)
                attributes.push((name, "".to_string()));
            }
        }

        attributes
    }

    fn parse_end_tag(&mut self) -> Token {
        self.position += 1; // Skip '/'
        let tag_name = self.parse_tag_name();
        self.skip_until('>');
        Token::EndTag(tag_name)
    }

    fn parse_doctype_or_comment(&mut self) -> Token {
        self.position += 1; // Skip '!'

        if self.position + 2 >= self.input.len() {
            return Token::Text("<!".to_string());
        }

        if &self.input[self.position..self.position + 2] == "--" {
            self.position += 2;
            self.parse_comment()
        } else {
            self.parse_doctype()
        }
    }

    fn parse_comment(&mut self) -> Token {
        let start = self.position;
        while self.position + 2 < self.input.len()
            && &self.input[self.position..self.position + 2] != "--"
        {
            self.position += 1;
        }

        let comment = self.input[start..self.position].to_string();
        self.position += 3; // Skip '-->'
        Token::Comment(comment)
    }

    fn parse_doctype(&mut self) -> Token {
        // Skip until '>'
        while self.position < self.input.len()
            && self.input.chars().nth(self.position).unwrap() != '>'
        {
            self.position += 1;
        }
        self.position += 1; // Skip '>'
        Token::Doctype
    }

    fn parse_text(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if c == '<' {
                break;
            }
            self.position += 1;
        }
        let text = self.input[start..self.position].trim().to_string();
        if text.is_empty() {
            self.next_token()
        } else {
            Token::Text(text)
        }
    }

    fn parse_tag_name(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len() {
            let c = self.input.chars().nth(self.position).unwrap();
            if !c.is_alphanumeric() {
                break;
            }
            self.position += 1;
        }
        self.input[start..self.position].to_ascii_lowercase()
    }

    fn skip_until(&mut self, c: char) {
        while self.position < self.input.len() {
            if self.input.chars().nth(self.position).unwrap() == c {
                self.position += 1;
                break;
            }
            self.position += 1;
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self
                .input
                .chars()
                .nth(self.position)
                .unwrap()
                .is_whitespace()
        {
            self.position += 1;
        }
    }
}
