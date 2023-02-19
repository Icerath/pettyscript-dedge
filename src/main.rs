use std::fs;

use nom_supreme::error::ErrorTree;

mod ast;
mod error;
mod interpreter;
mod parser;

pub type NomErr<'a> = nom_supreme::error::ErrorTree<&'a str>;
pub type IRes<'a, T = ast::Node, E = NomErr<'a>> = nom::IResult<&'a str, T, E>;

fn main() {
    let input = fs::read_to_string("example.pty").expect("Failed to read input");
    let ast = match parser::parse(&input) {
        Ok(ast) => ast,
        Err(err) => {
            print_error(&input, err);
            return;
        }
    };
    println!("{ast:#?}");
}
fn print_error(original_input: &str, err: ErrorTree<&str>) {
    let ErrorTree::Base { location, kind } = err else {
        panic!("{err:?}: Invalid Error Kind!\n");
    };
    let index = original_input.len() - location.len();
    assert_eq!(&original_input[index..], location);
    let line_before_location_index = original_input[..index].trim().rfind('\n').unwrap_or(0);
    let line_before_location = &original_input[line_before_location_index..].trim();
    let line = line_before_location
        .lines()
        .next()
        .unwrap_or(line_before_location);
    let index_in_line = line_before_location.len() - location.len();
    println!("{kind}\n'{line:?}'");
    println!("  {}^", ".".repeat(index_in_line));
}
