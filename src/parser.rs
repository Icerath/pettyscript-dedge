use crate::ast::Node;
use crate::{IRes, NomErr};
use nom::{
    branch::alt,
    bytes::complete::{escaped, is_a, tag, take_while},
    character::{
        complete::{alphanumeric1, char, digit0, digit1, one_of},
        is_digit,
    },
    combinator::{consumed, cut, map, map_res, opt, recognize, value},
    error::ErrorKind,
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree, Expectation},
    final_parser::final_parser,
    ParserExt,
};

pub fn parse(input: &str) -> Result<Node, NomErr> {
    todo!()
}
