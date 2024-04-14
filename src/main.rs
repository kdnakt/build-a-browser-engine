extern crate image;

use std::collections::HashMap;
use std::path::Path;
use std::fs::File;

mod css;
mod dom;
mod parser;
mod style;
mod layout;
mod painting;

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

    let html = "<html lang='ja' data-theme='light'><body><h1>Title</h1></body></html>";
    // let mut parser = parser::Parser::new(html.to_string());
    // println!("{}", parser.consume_char());
    println!("parsed: {:?}", parser::parse(html.to_string()));

    let read_source = |path: &str| {
        std::fs::read_to_string(path).unwrap()
    };
    let html = read_source("examples/test.html");
    let css = read_source("examples/test.css");
    let initial_containing_block = layout::Dimensions {
        content: layout::Rect { x: 0.0, y: 0.0, width: 800.0, height: 600.0 },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    let root_node = parser::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, initial_containing_block);

    let canvas = painting::paint(&layout_root, initial_containing_block.content);

    let file = File::create(&Path::new("output.png")).unwrap();
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let buffer: Vec<image::Rgba<u8>> = unsafe { std::mem::transmute(canvas.pixels) };
}
