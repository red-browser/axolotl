use super::rules::*;
use super::values::*;

pub struct CssParser {
    input: String,
    position: usize,
}

impl CssParser {
    pub fn new(input: String) -> Self {
        CssParser { input, position: 0 }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet { rules: Vec::new() };

        while !self.eof() {
            self.consume_whitespace();
            if self.eof() {
                break;
            }

            if let Some(rule) = self.parse_rule() {
                stylesheet.rules.push(rule);
            }
        }

        stylesheet
    }

    fn parse_rule(&mut self) -> Option<Rule> {
        let selectors = self.parse_selectors();
        let declarations = self.parse_declarations();

        Some(Rule::Style(StyleRule {
            selectors,
            declarations,
        }))
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        loop {
            selectors.push(Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();

            if self.next_char() == ',' {
                self.consume_char();
                self.consume_whitespace();
            } else if self.next_char() == '{' {
                break;
            }
        }

        selectors
    }

    fn parse_simple_selector(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector::new();

        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.classes.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                    selector.universal = true;
                }
                '[' => {
                    self.consume_char();
                    selector.attributes.push(self.parse_attribute_selector());
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break,
            }
        }

        selector
    }

    fn parse_attribute_selector(&mut self) -> AttributeSelector {
        let name = self.parse_identifier();
        let mut op = None;
        let mut value = None;

        self.consume_whitespace();
        if self.next_char() == ']' {
            self.consume_char();
            return AttributeSelector { name, op, value };
        }

        // Parse operator
        if self.next_char() == '=' {
            op = Some(AttributeOperator::Equal);
            self.consume_char();
        } else {
            let op_str = self.consume_while(|c| matches!(c, '~' | '|' | '^' | '$' | '*'));
            if !op_str.is_empty() && self.next_char() == '=' {
                op = match op_str.as_str() {
                    "~" => Some(AttributeOperator::Includes),
                    "|" => Some(AttributeOperator::DashMatch),
                    "^" => Some(AttributeOperator::Prefix),
                    "$" => Some(AttributeOperator::Suffix),
                    "*" => Some(AttributeOperator::Substring),
                    _ => None,
                };
                self.consume_char();
            }
        }

        // Parse value if present
        if op.is_some() {
            self.consume_whitespace();
            let quote = self.next_char();
            if quote == '\'' || quote == '"' {
                self.consume_char();
                value = Some(self.consume_while(|c| c != quote));
                self.consume_char();
            } else {
                value = Some(self.parse_identifier());
            }
        }

        self.consume_whitespace();
        if self.next_char() == ']' {
            self.consume_char();
        }

        AttributeSelector { name, op, value }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        assert_eq!(self.consume_char(), '{');
        let mut declarations = Vec::new();

        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }

            if let Some(declaration) = self.parse_declaration() {
                declarations.push(declaration);
            }
        }

        declarations
    }

    fn parse_declaration(&mut self) -> Option<Declaration> {
        let property_name = self.parse_identifier();
        self.consume_whitespace();

        if self.consume_char() != ':' {
            return None;
        }

        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();

        let important = if self.next_char() == '!' {
            self.consume_char();
            let important_str = self.parse_identifier();
            important_str.to_lowercase() == "important"
        } else {
            false
        };

        if self.next_char() == ';' {
            self.consume_char();
        }

        Some(Declaration {
            name: property_name,
            value,
            important,
        })
    }

    fn parse_value(&mut self) -> Value {
        let mut values = Vec::new();
        loop {
            self.consume_whitespace();
            let value = match self.next_char() {
                '0'..='9' => {
                    let num = self.parse_float();
                    if self.next_char() == '%' {
                        self.consume_char();
                        Value::Percentage(num)
                    } else {
                        Value::Length(num, self.parse_unit())
                    }
                }
                '#' => self.parse_color(),
                '"' | '\'' => {
                    let quote = self.consume_char();
                    let s = self.consume_while(|c| c != quote);
                    self.consume_char();
                    Value::String(s)
                }
                _ => Value::Keyword(self.parse_identifier()),
            };
            values.push(value);

            if self.next_char() != ' ' {
                break;
            }
            self.consume_char();
        }

        if values.len() == 1 {
            values.remove(0)
        } else {
            Value::List(values)
        }
    }

    fn parse_length(&mut self) -> Value {
        let num = self.parse_float();
        let unit = self.parse_unit();
        Value::Length(num, unit)
    }

    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| matches!(c, '0'..='9' | '.'));
        s.parse().unwrap_or(0.0)
    }

    fn parse_unit(&mut self) -> Unit {
        let unit_str = self.parse_identifier().to_lowercase();
        match unit_str.as_str() {
            "px" => Unit::Px,
            "em" => Unit::Em,
            "rem" => Unit::Rem,
            "%" => Unit::Percent,
            "" => Unit::Px, // Default to pixels for unitless numbers
            _ => Unit::Px,
        }
    }

    fn parse_color(&mut self) -> Value {
        assert_eq!(self.consume_char(), '#');
        let hex = self.consume_while(|c| c.is_ascii_hexdigit());

        match hex.len() {
            3 => Value::Color(Color {
                r: u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0),
                g: u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0),
                b: u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0),
                a: 1.0,
            }),
            6 => Value::Color(Color {
                r: u8::from_str_radix(&hex[0..2], 16).unwrap_or(0),
                g: u8::from_str_radix(&hex[2..4], 16).unwrap_or(0),
                b: u8::from_str_radix(&hex[4..6], 16).unwrap_or(0),
                a: 1.0,
            }),
            8 => Value::Color(Color {
                r: u8::from_str_radix(&hex[0..2], 16).unwrap_or(0),
                g: u8::from_str_radix(&hex[2..4], 16).unwrap_or(0),
                b: u8::from_str_radix(&hex[4..6], 16).unwrap_or(0),
                a: u8::from_str_radix(&hex[6..8], 16).unwrap_or(255) as f32 / 255.0,
            }),
            _ => Value::Keyword("transparent".to_string()),
        }
    }

    // Helper methods
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.position..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.position += next_pos;
        cur_char
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn next_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap_or('\0')
    }

    fn eof(&self) -> bool {
        self.position >= self.input.len()
    }
}

fn valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_'
}
