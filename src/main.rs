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

            if elem.self_closing {
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

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let html = if args.len() > 1 {
        fs::read_to_string(&args[1])?
    } else {
        fs::read_to_string("tests/html/test.html")?
    };

    let mut parser = Parser::new(html);
    let dom = parser.parse();
    pretty_print(&dom, 0);
    Ok(())
}
