use super::values::Value;

#[derive(Debug, PartialEq, Clone)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Rule {
    Style(StyleRule),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StyleRule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub name: String,
    pub value: Value,
    pub important: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Selector {
    Simple(SimpleSelector),
}

#[derive(Debug, PartialEq, Clone, Eq)] // Added Eq
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: Vec<AttributeSelector>,
    pub universal: bool,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct AttributeSelector {
    pub name: String,
    pub op: Option<AttributeOperator>,
    pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub enum AttributeOperator {
    Equal,
    Includes,
    DashMatch,
    Prefix,
    Suffix,
    Substring,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Specificity(u32, u32, u32);

impl Ord for Specificity {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.cmp(&other.0) {
            std::cmp::Ordering::Equal => match self.1.cmp(&other.1) {
                std::cmp::Ordering::Equal => self.2.cmp(&other.2),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl SimpleSelector {
    pub fn new() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
            attributes: Vec::new(),
            universal: false,
        }
    }

    pub fn specificity(&self) -> Specificity {
        let a = if self.id.is_some() { 1 } else { 0 };
        let b = self.classes.len() as u32;
        let c = if self.tag_name.is_some() { 1 } else { 0 };
        Specificity(a, b, c)
    }
}
