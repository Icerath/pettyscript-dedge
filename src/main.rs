use ast::Node;
use nom_supreme::error::ErrorTree;

mod ast;
mod error;
mod parser;
mod rc_str;
mod slim_rc;
mod vm;

pub type NomErr<'a> = nom_supreme::error::ErrorTree<&'a str>;
pub type IRes<'a, T = ast::Node, E = NomErr<'a>> = nom::IResult<&'a str, T, E>;

fn main() -> Result<(), ()> {
    let input = include_str!("../example.pty");
    let ast = read_ast(input)?;
    vm::run_virtual_machine(&ast);
    Ok(())
}
fn read_ast(input: &str) -> Result<Node, ()> {
    match parser::parse(input) {
        Ok(ast) => return Ok(ast),
        Err(err) => print_error(input, err),
    }
    Err(())
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
