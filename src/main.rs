pub mod constraints;

fn main() {
    let x = &*constraints::REMAPPED_WORDS;
    println!("{:#?}", x);
}
