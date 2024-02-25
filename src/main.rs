use std::collections::HashMap;

mod dom;
mod parser;

fn main() {
    println!("Hello, world!");

    let text = dom::text("Hello World".to_string());
    println!("{:?}", text);

    let mut attrs = HashMap::new();
    attrs.insert(
        "lang".to_string(),
        "en".to_string()
    );
    attrs.insert(
        "data-theme".to_string(),
        "dark".to_string()
    );
    let mut children = Vec::new();
    children.push(text);
    let html = dom::elem("html".to_string(), attrs, children);
    println!("{:?}", html);

    let html = "<html><body><h1>Title</h1></body></html>";
    let mut parser = parser::Parser::new(html.to_string());
    // println!("{}", parser.consume_char());
    println!("parsed: {:?}", parser.parse_node());
}
