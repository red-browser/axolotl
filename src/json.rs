use crate::css::rules::{
    AttributeOperator, AttributeSelector, Declaration, Keyframe, Rule, Selector, SimpleSelector,
    StyleRule, Stylesheet,
};
use crate::css::values::Value as CssValue;
use crate::html::dom::{ElementData, Node, NodeType};
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
                "attributes": elem.attributes.iter().map(|(k, v)| json!({k: v})).collect::<Vec<_>>(),
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
        Selector::Simple(simple) => {
            json!({
                "type": "simple_selector",
                "tag_name": simple.tag_name,
                "id": simple.id,
                "classes": simple.classes,
                "universal": simple.universal,
                "attributes": simple.attributes.iter().map(attribute_selector_to_json).collect::<Vec<_>>()
            })
        }
    }
}

fn attribute_selector_to_json(attr: &AttributeSelector) -> Value {
    json!({
        "name": attr.name,
        "operator": match attr.op {
            Some(AttributeOperator::Equal) => "=",
            Some(AttributeOperator::Includes) => "~=",
            Some(AttributeOperator::DashMatch) => "|=",
            Some(AttributeOperator::Prefix) => "^=",
            Some(AttributeOperator::Suffix) => "$=",
            Some(AttributeOperator::Substring) => "*=",
            None => "",
        },
        "value": attr.value
    })
}

fn declaration_to_json(declaration: &Declaration) -> Value {
    json!({
        "property": declaration.name,
        "value": css_value_to_json(&declaration.value),
        "important": declaration.important
    })
}

fn css_value_to_json(value: &CssValue) -> Value {
    match value {
        CssValue::Keyword(s) => json!(s),
        CssValue::Length(n, unit) => json!({
            "value": n,
            "unit": format!("{:?}", unit).to_lowercase()
        }),
        CssValue::Percentage(p) => json!({
            "value": p,
            "unit": "%"
        }),
        CssValue::Color(color) => json!({
            "type": "color",
            "r": color.r,
            "g": color.g,
            "b": color.b,
            "a": color.a
        }),
        CssValue::Url(url) => json!(url),
        CssValue::String(s) => json!(s),
        CssValue::Function(name, args) => json!({
            "type": "function",
            "name": name,
            "args": args.iter().map(css_value_to_json).collect::<Vec<_>>()
        }),
        CssValue::Rect(rect) => json!({
            "type": "rect",
            "top": css_value_to_json(&rect.top),
            "right": css_value_to_json(&rect.right),
            "bottom": css_value_to_json(&rect.bottom),
            "left": css_value_to_json(&rect.left)
        }),
        CssValue::Initial => json!("initial"),
        CssValue::Inherit => json!("inherit"),
        CssValue::Unset => json!("unset"),
        CssValue::CurrentColor => json!("currentColor"),
        CssValue::Auto => json!("auto"),
        CssValue::None => json!("none"),
        CssValue::LinearGradient(gradient) => json!({
            "type": "linear-gradient",
            "direction": css_value_to_json(&gradient.direction),
            "stops": gradient.stops.iter().map(|stop| json!({
                "color": css_value_to_json(&CssValue::Color(stop.color.clone())),
                "position": stop.position.as_ref().map(css_value_to_json)
            })).collect::<Vec<_>>()
        }),
        CssValue::List(values) => json!(values.iter().map(css_value_to_json).collect::<Vec<_>>()),
    }
}

fn keyframe_to_json(keyframe: &Keyframe) -> Value {
    json!({
        "selectors": keyframe.selectors,
        "declarations": keyframe.declarations.iter().map(declaration_to_json).collect::<Vec<_>>()
    })
}
