#[macro_use]
extern crate nom;

mod parser;
mod streamcontent;

use streamcontent::StreamContentVisitor;

fn main() {
    let input = include_bytes!("input");
    let top_group = parser::group(input).to_result().unwrap();
    let mut visitor = StreamContentVisitor::new();
    top_group.visit(&mut visitor);
    println!("Part 1: Total score: {}", visitor.score());
    println!("Part 2: Total amount of garbage: {}", visitor.garbage_amount());
}
