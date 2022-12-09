pub mod constraints;
pub mod parse;

fn main() {
    let x = parse::parse("Hello world");
    // println!("{:#?}", x);
}
