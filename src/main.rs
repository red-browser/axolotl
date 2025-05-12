mod css;
mod html;

use crate::html::dom::NodeType;
use html::dom::Node;
use html::Parser;
use std::env;
use std::fs;

fn pretty_print(node: &Node, indent: usize) {
    let spaces = " ".repeat(indent);

    match &node.node_type {
        NodeType::Element(elem) => {
            print!("{}<{}", spaces, elem.tag_name);
            for (name, value) in &elem.attributes {
                if value.is_empty() {
                    print!(" {}", name);
                } else {
                    print!(" {}=\"{}\"", name, value);
                }
            }

            if html::dom::is_void_element(&elem.tag_name) {
                println!("/>");
                return;
            }

            println!(">");

            for child in &node.children {
                pretty_print(child, indent + 2);
            }

            println!("{}</{}>", spaces, elem.tag_name);
        }
        NodeType::Text(text) => {
            if !text.trim().is_empty() {
                println!("{}{}", spaces, text.trim());
            }
        }
    }
}

// fn main() {
//     let html = r#"
//         <!DOCTYPE html>
//         <html lang="en">
//             <head>
//                 <title class="main-title">Test Page</title>
//             </head>
//             <body>
//                 <h1 id="header" data-test>Hello, world!</h1>
//                 <p class="content" hidden>This is a test page.</p>
//             </body>
//         </html>
//     "#.to_string();

//     let mut parser = Parser::new(html);
//     let dom = parser.parse();

//     pretty_print(&dom, 0);
// }

fn parse_inline_styles(node: &Node, stylesheet: &mut css::rules::Stylesheet) {
    match &node.node_type {
        NodeType::Element(elem) if elem.tag_name == "style" => {
            if let Some(text_node) = node.children.first() {
                if let NodeType::Text(css_text) = &text_node.node_type {
                    let mut css_parser = css::parser::CssParser::new(css_text.clone());
                    stylesheet.rules.extend(css_parser.parse_stylesheet().rules);
                }
            }
        }
        _ => {
            for child in &node.children {
                parse_inline_styles(child, stylesheet);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let html = if args.len() > 1 {
        fs::read_to_string(&args[1])?
    } else {
        fs::read_to_string("tests/html/test.html")?
    };

    let mut html_parser = Parser::new(html);
    let dom = html_parser.parse();

    let css = if args.len() > 1 {
        fs::read_to_string(&args[1])?
    } else {
        fs::read_to_string("tests/css/test.css")?
    };

    let mut css_parser = css::parser::CssParser::new(css.to_string());
    let stylesheet = css_parser.parse_stylesheet();

    println!("Parsed CSS Rules:");
    for rule in stylesheet.rules {
        if let css::rules::Rule::Style(style_rule) = rule {
            println!("Selectors: {:?}", style_rule.selectors);
            for decl in style_rule.declarations {
                println!("  {}: {:?}", decl.name, decl.value);
            }
        }
    }

    for child in &dom.children {
        pretty_print(child, 0);
    }

    Ok(())
}
