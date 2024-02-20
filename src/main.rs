use std::collections::HashMap;

mod dom;

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
}
