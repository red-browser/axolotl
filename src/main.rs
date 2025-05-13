mod css;
mod html;

use crate::html::dom::NodeType;
use html::dom::Node;
use html::Parser;
use std::env;
use std::fs;
use std::path::Path;

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
                    let mut css_parser = css::parser::CssParser::new(css_text.as_str());
                    if let Ok(parsed) = css_parser.parse_stylesheet() {
                        stylesheet.rules.extend(parsed.rules);
                    }
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
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path)?;

    let path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => {
            let mut html_parser = Parser::new(content);
            let dom = html_parser.parse();
            pretty_print(&dom, 0);
        }
        Some("css") => {
            let mut css_parser = css::parser::CssParser::new(&content);
            match css_parser.parse_stylesheet() {
                Ok(stylesheet) => {
                    println!("Parsed CSS Rules:");
                    for rule in stylesheet.rules {
                        match rule {
                            css::rules::Rule::Style(style_rule) => {
                                println!("Selectors: {:?}", style_rule.selectors);
                                for decl in style_rule.declarations {
                                    println!("  {}: {:?}", decl.name, decl.value);
                                }
                            }
                            css::rules::Rule::Media { query, rules } => {
                                println!("@media {} {{", query);
                                println!("}}");
                            }
                            css::rules::Rule::Keyframes { name, frames } => {
                                println!("@keyframes {} {{", name);
                                println!("}}");
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing CSS: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unsupported file type. Please use .html or .css");
            std::process::exit(1);
        }
    }

    Ok(())
}
