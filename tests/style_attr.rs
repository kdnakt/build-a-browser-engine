// integration test

extern crate robinson;

use std::collections::HashMap;

#[test]
fn test_parse_style_attr() {
    use robinson::{parser, dom};

    let html = "<html style=\"color: red;\">Title</html>".to_string();
    let parsed = parser::parse(html);
    let element_data = match parsed.node_type {
        dom::NodeType::Element(ref element_data) => element_data,
        _ => panic!("element not found"),
    };
    let stylesheet = element_data.style();
    println!("{:?}", stylesheet);

    // TODO: assert
}
