extern crate image;
extern crate getopts;

use getopts::Options;
use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::env;

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

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("o", "output", "Output file", "FILENAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string())
    };

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

    let filename = matches.opt_str("o").unwrap_or("output.png".to_string());
    let mut file = File::create(&Path::new(&filename)).unwrap();
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let buffer: Vec<image::Rgba<u8>> = unsafe { std::mem::transmute(canvas.pixels) };
    let img = image::ImageBuffer::from_fn(w, h, Box::new(|x: u32, y: u32| buffer[(y * w + x) as usize]));

    let result = image::DynamicImage::ImageRgba8(img).save(&mut file, image::PNG);
    match result {
        Ok(_) => println!("Saved output"),
        Err(_) => println!("Error")
    }
}
