mod html;

use html::{Node, Parser};

fn pretty_print(node: &Node, indent: usize) {
    let spaces = " ".repeat(indent);

    match &node.node_type {
        html::dom::NodeType::Element(elem) => {
            println!("{}<{}>", spaces, elem.tag_name);
            for child in &node.children {
                pretty_print(child, indent + 2);
            }
            println!("{}</{}>", spaces, elem.tag_name);
        }
        html::dom::NodeType::Text(text) => {
            println!("{}{}", spaces, text);
        }
    }
}

fn main() {
    let html = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Test Page</title>
            </head>
            <body>
                <h1>Hello, world!</h1>
                <p>This is a test page.</p>
            </body>
        </html>
    "#;

    let mut parser = Parser::new(html.to_string());
    let dom = parser.parse();

    pretty_print(&dom, 0);
}
