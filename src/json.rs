use crate::css::rules::{Declaration, Rule, Selector, SimpleSelector, StyleRule, Stylesheet};
use crate::html::dom::{ElementData, Node, NodeType};
use serde::{Serialize, Serializer};
use serde_json::{json, Value};

pub fn node_to_json(node: &Node) -> Value {
    match &node.node_type {
        NodeType::Document => json!({
            "type": "document",
            "children": node.children.iter().map(node_to_json).collect::<Vec<_>>()
        }),
        NodeType::Doctype => json!({
            "type": "doctype"
        }),
        NodeType::Element(elem) => {
            let mut json_elem = json!({
                "type": "element",
                "tag": elem.tag_name,
                "attributes": elem.attributes,
                "children": node.children.iter().map(node_to_json).collect::<Vec<_>>()
            });

            if elem.is_self_closing {
                json_elem["selfClosing"] = Value::Bool(true);
            }

            json_elem
        }
        NodeType::Text(text) => json!({
            "type": "text",
            "value": text
        }),
        NodeType::Comment(text) => json!({
            "type": "comment",
            "value": text
        }),
    }
}

pub fn stylesheet_to_json(stylesheet: &Stylesheet) -> Value {
    json!({
        "type": "stylesheet",
        "rules": stylesheet.rules.iter().map(rule_to_json).collect::<Vec<_>>()
    })
}

fn rule_to_json(rule: &Rule) -> Value {
    match rule {
        Rule::Style(style_rule) => json!({
            "type": "style_rule",
            "selectors": style_rule.selectors.iter().map(selector_to_json).collect::<Vec<_>>(),
            "declarations": style_rule.declarations.iter().map(declaration_to_json).collect::<Vec<_>>()
        }),
        Rule::Media { query, rules } => json!({
            "type": "media_rule",
            "query": query,
            "rules": rules.iter().map(rule_to_json).collect::<Vec<_>>()
        }),
        Rule::Keyframes { name, frames } => json!({
            "type": "keyframes_rule",
            "name": name,
            "frames": frames.iter().map(keyframe_to_json).collect::<Vec<_>>()
        }),
    }
}

fn selector_to_json(selector: &Selector) -> Value {
    match selector {
        Selector::Simple(simple) => json!({
            "type": "simple_selector",
            "tag_name": simple.tag_name,
            "id": simple.id,
            "classes": simple.classes,
            "universal": simple.universal
        }),
    }
}

fn declaration_to_json(declaration: &Declaration) -> Value {
    json!({
        "property": declaration.name,
        "value": format!("{:?}", declaration.value),
        "important": declaration.important
    })
}

fn keyframe_to_json(keyframe: &Keyframe) -> Value {
    json!({
        "selectors": keyframe.selectors,
        "declarations": keyframe.declarations.iter().map(declaration_to_json).collect::<Vec<_>>()
    })
}
