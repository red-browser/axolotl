mod css;
mod html;

use crate::html::dom::NodeType;
use html::dom::Node;
use html::Parser;
use std::env;
use std::fs;
use std::path::Path;

/* defining two printmodes */
#[derive(Clone, Copy, PartialEq)]
pub enum PrintMode {
    Pretty,
    Compact,
}

fn escape_html_text(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_html_attribute(value: &str) -> String {
    escape_html_text(value)
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn collapse_whitespace(text: String) -> String {
    let mut result = String::with_capacity(text.len());
    let mut in_whitespace = false;

    for c in text.chars() {
        if c.is_whitespace() {
            if !in_whitespace {
                result.push(' ');
                in_whitespace = true;
            }
        } else {
            result.push(c);
            in_whitespace = false;
        }
    }

    result
}

fn pretty_print(node: &Node, indent: usize, mode: PrintMode) {
    let spaces = match mode {
        PrintMode::Pretty => " ".repeat(indent),
        PrintMode::Compact => "".to_string(),
    };

    match &node.node_type {
        NodeType::Document => {
            for child in &node.children {
                pretty_print(child, indent, mode);
            }
        }
        NodeType::Doctype => {
            println!("<!DOCTYPE html>");
        }
        NodeType::Element(elem) => {
            // Opening tag
            match mode {
                PrintMode::Pretty => print!("{}<{}", spaces, elem.tag_name),
                PrintMode::Compact => print!("<{}", elem.tag_name),
            }

            // Attributes
            for (name, value) in &elem.attributes {
                if value.is_empty() {
                    print!(" {}", name);
                } else {
                    print!(" {}=\"{}\"", name, escape_html_attribute(value));
                }
            }

            // Self-closing or children
            if elem.is_self_closing || html::dom::is_void_element(&elem.tag_name) {
                println!("/>");
                return;
            } else {
                print!(">");
                if mode == PrintMode::Pretty && !node.children.is_empty() {
                    println!();
                }
            }

            // Children
            for child in &node.children {
                pretty_print(child, indent + 2, mode);
            }

            // Closing tag
            match mode {
                PrintMode::Pretty if !node.children.is_empty() => {
                    println!("{}</{}>", spaces, elem.tag_name);
                }
                _ => print!("</{}>", elem.tag_name),
            }

            if mode == PrintMode::Pretty {
                println!();
            }
        }
        NodeType::Text(text) => {
            let text = escape_html_text(text);
            let text = match mode {
                PrintMode::Pretty => collapse_whitespace(text.trim().to_string()),
                PrintMode::Compact => collapse_whitespace(text),
            };
            if !text.is_empty() {
                match mode {
                    PrintMode::Pretty => println!("{}{}", spaces, text),
                    PrintMode::Compact => print!("{}", text),
                }
            }
        }
        NodeType::Comment(text) => match mode {
            PrintMode::Pretty => println!("{}<!--{}-->", spaces, text),
            PrintMode::Compact => print!("<!--{}-->", text),
        },
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
        eprintln!("Usage: {} [--format=pretty|compact] <file>", args[0]);
        std::process::exit(1);
    }

    let mut format = PrintMode::Compact;
    let mut file_index = 1;

    if args[1].starts_with("--format=") {
        match args[1].split('=').nth(1) {
            Some("pretty") => format = PrintMode::Pretty,
            Some("compact") => format = PrintMode::Compact,
            _ => {
                eprintln!("Invalid format. Use 'pretty' or 'compact'");
                std::process::exit(1);
            }
        }
        file_index = 2;
    }

    if args.len() <= file_index {
        eprintln!("Missing file argument");
        std::process::exit(1);
    }

    let file_path = &args[file_index];
    let content = fs::read_to_string(file_path)?;

    let path = Path::new(file_path);
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("html") => {
            let mut html_parser = Parser::new(content);
            let dom = html_parser.parse();
            pretty_print(&dom, 0, format);
        }
        Some("css") => {
            let mut css_parser = css::parser::CssParser::new(&content);
            match css_parser.parse_stylesheet() {
                Ok(stylesheet) => {
                    println!("Parsed CSS Rules:");
                    for rule in stylesheet.rules {
                        match rule {
                            css::rules::Rule::Style(style_rule) => {
                                println!("Style Rule:");
                                println!("  Selectors: {:?}", style_rule.selectors);
                                for decl in style_rule.declarations {
                                    println!("    {}: {:?}", decl.name, decl.value);
                                }
                            }
                            css::rules::Rule::Media { query, rules } => {
                                println!("@media {} {{", query);
                                for nested_rule in rules {
                                    if let css::rules::Rule::Style(sr) = nested_rule {
                                        println!("  Nested Rule:");
                                        println!("    Selectors: {:?}", sr.selectors);
                                        for decl in sr.declarations {
                                            println!("      {}: {:?}", decl.name, decl.value);
                                        }
                                    }
                                }
                                println!("}}");
                            }
                            css::rules::Rule::Keyframes { name, frames } => {
                                println!("@keyframes {} {{", name);
                                for frame in frames {
                                    println!("  Keyframe Selectors: {:?}", frame.selectors);
                                    for decl in frame.declarations {
                                        println!("    {}: {:?}", decl.name, decl.value);
                                    }
                                }
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
