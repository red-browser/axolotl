use super::rules::*;
use super::values::*;

const MAX_MEMORY_BYTES: usize = 10 * 1024 * 1024; // 10MB limit

pub struct CssParser<'a> {
    input: &'a str,
    position: usize,
    bytes_consumed: usize,
}

impl<'a> CssParser<'a> {
    pub fn new(input: &'a str) -> Self {
        CssParser {
            input,
            position: 0,
            bytes_consumed: 0,
        }
    }

    pub fn parse_stylesheet(&mut self) -> Result<Stylesheet, &'static str> {
        let mut stylesheet = Stylesheet { rules: Vec::new() };
        let mut consecutive_errors = 0;
        const MAX_CONSECUTIVE_ERRORS: usize = 10;

        while !self.eof() && consecutive_errors < MAX_CONSECUTIVE_ERRORS {
            let start_pos = self.position;
            self.consume_whitespace_and_comments();

            if self.eof() {
                break;
            }

            match self.next_char() {
                '@' => match self.parse_at_rule() {
                    Ok(rule) => {
                        stylesheet.rules.push(rule);
                        consecutive_errors = 0;
                    }
                    Err(e) => {
                        eprintln!("{} at position {}", e, self.position);
                        consecutive_errors += 1;
                    }
                },
                _ => match self.parse_rule() {
                    Ok(rule) => {
                        stylesheet.rules.push(rule);
                        consecutive_errors = 0;
                    }
                    Err(e) => {
                        eprintln!(
                            "Skipping malformed rule: {} at position {}",
                            e, self.position
                        );
                        self.skip_to_next_rule();
                        consecutive_errors += 1;
                    }
                },
            }

            if self.position == start_pos {
                eprintln!("Forcing progress at position {}", self.position);
                self.consume_char();
                consecutive_errors += 1;
            }
        }

        if consecutive_errors >= MAX_CONSECUTIVE_ERRORS {
            eprintln!("Too many errors, aborting parsing");
        }

        Ok(stylesheet)
    }

    fn skip_block(&mut self) -> Result<(), &'static str> {
        let mut depth = 1;
        self.consume_char();

        while depth > 0 && !self.eof() {
            match self.next_char() {
                '{' => {
                    depth += 1;
                    self.consume_char();
                }
                '}' => {
                    depth -= 1;
                    self.consume_char();
                }
                '"' | '\'' => {
                    let quote = self.consume_char();
                    self.consume_while(|c| c != quote);
                    self.consume_char();
                }
                '/' if self.peek_char(1) == '*' => {
                    self.skip_comment();
                }
                _ => {
                    self.consume_char();
                }
            }
        }

        if depth == 0 {
            Ok(())
        } else {
            Err("Unclosed block")
        }
    }

    fn check_memory_limit(&self) -> Result<(), &'static str> {
        if self.bytes_consumed > MAX_MEMORY_BYTES {
            Err("Memory limit exceeded")
        } else {
            Ok(())
        }
    }

    fn skip_at_rule(&mut self) -> Result<(), &'static str> {
        self.consume_char(); // Skip @
        let at_keyword = self.parse_identifier();

        self.consume_whitespace();

        // Skip until { or ; depending on at-rule type
        match at_keyword.as_str() {
            "media" | "keyframes" | "supports" => {
                // Skip the condition
                self.consume_while(|c| c != '{');
                if self.next_char() == '{' {
                    self.skip_block()?;
                }
            }
            _ => {
                self.consume_while(|c| c != ';');
                if self.next_char() == ';' {
                    self.consume_char();
                }
            }
        }

        Ok(())
    }

    fn skip_to_next_rule(&mut self) -> usize {
        let start = self.position;
        let mut depth = 0;

        while !self.eof() {
            match self.next_char() {
                '@' if depth == 0 => {
                    break;
                }
                '{' => {
                    depth += 1;
                    self.consume_char();
                }
                '}' => {
                    if depth > 0 {
                        depth -= 1;
                    }
                    self.consume_char();
                    if depth == 0 {
                        break;
                    } else {
                        continue;
                    }
                }
                _ => {
                    self.consume_char();
                }
            }
        }

        self.position - start
    }

    fn parse_rule(&mut self) -> Result<Rule, &'static str> {
        let start_pos = self.position;
        let selectors = self.parse_selectors()?;

        if self.position == start_pos {
            return Err("No progress made while parsing selectors");
        }

        let declarations = self.parse_declarations()?;
        Ok(Rule::Style(StyleRule {
            selectors,
            declarations,
        }))
    }

    fn parse_selectors(&mut self) -> Result<Vec<Selector>, &'static str> {
        let mut selectors = Vec::new();

        loop {
            match self.parse_selector() {
                Ok(selector) => {
                    selectors.push(selector);
                    self.consume_whitespace();

                    if self.next_char() == ',' {
                        self.consume_char();
                        self.consume_whitespace();
                    } else if self.next_char() == '{' {
                        break;
                    }
                }
                Err(_) => {
                    self.skip_to_next_selector();
                    if self.next_char() == '{' {
                        break;
                    }
                }
            }
        }

        if selectors.is_empty() {
            Err("No valid selectors found")
        } else {
            Ok(selectors)
        }
    }

    fn skip_to_next_selector(&mut self) {
        self.consume_while(|c| c != ',' && c != '{');
        if self.next_char() == ',' {
            self.consume_char();
        }
    }

    fn parse_selector(&mut self) -> Result<Selector, &'static str> {
        let mut selector = SimpleSelector::new();
        let mut has_parts = false;

        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                    has_parts = true;
                }
                '.' => {
                    self.consume_char();
                    selector.classes.push(self.parse_identifier());
                    has_parts = true;
                }
                '*' => {
                    self.consume_char();
                    selector.universal = true;
                    has_parts = true;
                }
                '[' => {
                    self.consume_char();
                    selector.attributes.push(self.parse_attribute_selector()?);
                    has_parts = true;
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                    has_parts = true;
                }
                _ => break,
            }
        }

        if has_parts {
            Ok(Selector::Simple(selector))
        } else {
            Err("Empty selector")
        }
    }

    fn parse_attribute_selector(&mut self) -> Result<AttributeSelector, &'static str> {
        let name = self.parse_identifier();
        let mut op = None;
        let mut value = None;

        self.consume_whitespace();
        if self.next_char() == ']' {
            self.consume_char();
            return Ok(AttributeSelector { name, op, value });
        }

        if self.next_char() == '=' {
            op = Some(AttributeOperator::Equal);
            self.consume_char();
        } else {
            let op_str = self.consume_while(|c| matches!(c, '~' | '|' | '^' | '$' | '*'));
            if !op_str.is_empty() && self.next_char() == '=' {
                op = match op_str {
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

        if op.is_some() {
            self.consume_whitespace();
            let quote = self.next_char();
            if quote == '\'' || quote == '"' {
                self.consume_char();
                value = Some(self.consume_while(|c| c != quote).to_string());
                self.consume_char();
            } else {
                value = Some(self.parse_identifier());
            }
        }

        self.consume_whitespace();
        if self.next_char() == ']' {
            self.consume_char();
            Ok(AttributeSelector { name, op, value })
        } else {
            self.skip_to_char(']');
            Err("Invalid attribute selector")
        }
    }

    fn parse_declarations(&mut self) -> Result<Vec<Declaration>, &'static str> {
        if self.consume_char() != '{' {
            return Err("Expected '{' for declarations block");
        }

        let mut declarations = Vec::new();

        loop {
            self.consume_whitespace_and_comments();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }

            match self.parse_declaration() {
                Ok(declaration) => declarations.push(declaration),
                Err(_) => {
                    self.skip_to_next_declaration();
                }
            }
        }

        Ok(declarations)
    }

    fn skip_to_next_declaration(&mut self) {
        self.consume_while(|c| c != ';' && c != '}');
        if self.next_char() == ';' {
            self.consume_char();
        }
    }

    fn skip_to_char(&mut self, end_char: char) {
        self.consume_while(|c| c != end_char);
        if self.next_char() == end_char {
            self.consume_char();
        }
    }

    fn parse_declaration(&mut self) -> Result<Declaration, &'static str> {
        let property_name = self.parse_identifier();
        if property_name.is_empty() {
            return Err("Empty property name");
        }

        self.consume_whitespace();
        if self.consume_char() != ':' {
            return Err("Expected ':' after property name");
        }

        self.consume_whitespace();
        let value = self.parse_value();
        self.consume_whitespace();

        let important = if self.next_char() == '!' {
            self.consume_char();
            let important_str = self.parse_identifier().to_lowercase();
            important_str == "important"
        } else {
            false
        };

        if self.next_char() == ';' {
            self.consume_char();
        }

        Ok(Declaration {
            name: property_name,
            value,
            important,
        })
    }

    fn parse_value(&mut self) -> Value {
        let mut values = Vec::new();

        while !self.eof() {
            self.consume_whitespace();
            if self.next_char() == ';' || self.next_char() == '!' || self.next_char() == '}' {
                break;
            }

            let value = match self.next_char() {
                '0'..='9' | '.' | '+' | '-' => {
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
                    let s = self.consume_while(|c| c != quote).to_string();
                    self.consume_char();
                    Value::String(s)
                }
                '(' => {
                    self.consume_char();
                    let func_name = self.parse_identifier();
                    let args = self.parse_value_list(')');
                    Value::Function(func_name, args)
                }
                _ => {
                    let ident = self.parse_identifier();
                    if ident.is_empty() {
                        break;
                    }
                    match ident.to_lowercase().as_str() {
                        "rgb" | "rgba" | "hsl" | "hsla" => {
                            self.consume_char(); // Skip (
                            let args = self.parse_value_list(')');
                            Value::Function(ident, args)
                        }
                        _ => Value::Keyword(ident),
                    }
                }
            };

            values.push(value);
        }

        if values.len() == 1 {
            values.remove(0)
        } else {
            Value::List(values)
        }
    }

    fn parse_value_list(&mut self, end_char: char) -> Vec<Value> {
        let mut values = Vec::new();

        while !self.eof() {
            self.consume_whitespace();
            if self.next_char() == end_char {
                self.consume_char();
                break;
            }

            if self.next_char() == ',' {
                self.consume_char();
                self.consume_whitespace();
                continue;
            }

            values.push(self.parse_value());
        }

        values
    }

    fn parse_at_rule(&mut self) -> Result<Rule, &'static str> {
        let start_pos = self.position;
        self.consume_char(); // Skip @
        let name = self.parse_identifier();
        self.consume_whitespace();

        match name.as_str() {
            "media" => {
                let query = self.consume_while(|c| c != '{').trim().to_string();
                if query.is_empty() {
                    return Err("Empty media query");
                }
                self.expect_char('{')?;
                let rules = self.parse_rules_block()?;
                Ok(Rule::Media { query, rules })
            }
            "keyframes" => {
                let name = self.parse_identifier();
                self.expect_char('{')?;
                let frames = self.parse_keyframe_rules()?;
                Ok(Rule::Keyframes { name, frames })
            }
            _ => {
                self.consume_while(|c| c != ';' && c != '{');
                if self.next_char() == '{' {
                    self.skip_block()?;
                } else if self.next_char() == ';' {
                    self.consume_char();
                }
                Err("Skipped unsupported at-rule")
            }
        }
    }

    fn expect_char(&mut self, expected: char) -> Result<(), &'static str> {
        if self.next_char() == expected {
            self.consume_char();
            Ok(())
        } else {
            Err("Expected character not found")
        }
    }

    fn parse_rules_block(&mut self) -> Result<Vec<Rule>, &'static str> {
        let mut rules = Vec::new();
        self.consume_whitespace_and_comments();

        while !self.eof() && self.next_char() != '}' {
            match self.parse_rule() {
                Ok(rule) => rules.push(rule),
                Err(e) => {
                    eprintln!("Skipping malformed rule in block: {}", e);
                    self.skip_to_next_rule();
                }
            }
            self.consume_whitespace_and_comments();
        }

        self.expect_char('}')?;
        Ok(rules)
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
            4 => Value::Color(Color {
                r: u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap_or(0),
                g: u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap_or(0),
                b: u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap_or(0),
                a: u8::from_str_radix(&hex[3..4].repeat(2), 16).unwrap_or(255) as f32 / 255.0,
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

    fn parse_float(&mut self) -> f32 {
        let s = self.consume_while(|c| matches!(c, '0'..='9' | '.' | '+' | '-'));
        s.parse().unwrap_or(0.0)
    }

    fn parse_keyframe_rules(&mut self) -> Result<Vec<Keyframe>, &'static str> {
        let mut frames = Vec::new();
        while !self.eof() && self.next_char() != '}' {
            let selectors = self.parse_keyframe_selectors()?;
            let declarations = self.parse_declarations()?;
            frames.push(Keyframe {
                selectors,
                declarations,
            });
            self.consume_whitespace_and_comments();
        }
        self.expect_char('}')?;
        Ok(frames)
    }

    fn parse_keyframe_selectors(&mut self) -> Result<Vec<String>, &'static str> {
        let mut selectors = Vec::new();
        loop {
            self.consume_whitespace();
            selectors.push(self.consume_while(|c| c != ',' && c != '{').to_string());
            match self.next_char() {
                ',' => {
                    self.consume_char();
                    continue;
                }
                '{' => break,
                _ => return Err("Invalid keyframe selector"),
            }
        }
        Ok(selectors)
    }

    fn parse_unit(&mut self) -> Unit {
        let unit_str = self.parse_identifier().to_lowercase();
        match unit_str.as_str() {
            "px" => Unit::Px,
            "em" => Unit::Em,
            "rem" => Unit::Rem,
            "%" => Unit::Percent,
            "deg" => Unit::Deg,
            "rad" => Unit::Rad,
            "turn" => Unit::Turn,
            "s" => Unit::S,
            "ms" => Unit::Ms,
            "hz" => Unit::Hz,
            "dpi" => Unit::Dpi,
            _ => Unit::Px, // Default to pixels
        }
    }

    fn consume_whitespace_and_comments(&mut self) {
        loop {
            self.consume_whitespace();
            if self.next_char() == '/' && self.peek_char(1) == '*' {
                self.skip_comment();
            } else {
                break;
            }
        }
    }

    fn skip_comment(&mut self) {
        assert_eq!(self.consume_char(), '/');
        assert_eq!(self.consume_char(), '*');
        self.consume_while(|c| c != '*');
        if self.next_char() == '*' && self.peek_char(1) == '/' {
            self.consume_char();
            self.consume_char();
        }
    }

    fn peek_char(&self, offset: usize) -> char {
        self.input[self.position..]
            .chars()
            .nth(offset)
            .unwrap_or('\0')
    }

    // Helper methods
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.position..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.position += next_pos;
        cur_char
    }

    fn consume_while<F>(&mut self, test: F) -> &'a str
    where
        F: Fn(char) -> bool,
    {
        let start = self.position;
        while !self.eof() && test(self.next_char()) {
            self.position += self.next_char().len_utf8();
            self.bytes_consumed += self.next_char().len_utf8();
        }
        &self.input[start..self.position]
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c.is_whitespace());
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char).to_string()
    }

    fn next_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap_or('\0')
    }

    fn eof(&self) -> bool {
        self.position >= self.input.len()
    }
}

fn valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '-' || c == '_' || c as u32 > 0x7f
}
