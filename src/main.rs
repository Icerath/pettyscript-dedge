mod ast;
mod interpreter;
mod parser;

pub type NomErr<'a> = nom_supreme::error::ErrorTree<&'a str>;
pub type IRes<'a, T = ast::Node, E = NomErr<'a>> = nom::IResult<&'a str, T, E>;

fn main() {
    println!("Hello, world!");
}
