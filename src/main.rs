mod dom;

fn main() {
    println!("Hello, world!");

    let text = dom::text("Hello World".to_string());
    println!("{:?}", text);
}
