mod bin_expr;
mod statements;
mod tests;

use bin_expr::bin_expr;
use statements::statement;

use crate::{
    ast::{BinOp, Literal, Node, UnaryOp},
    error::PettyParseError,
};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, digit1, one_of},
    combinator::{cut, map, opt, recognize, value},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use nom_supreme::ParserExt;
use nom_supreme::{
    error::{BaseErrorKind, ErrorTree},
    final_parser::final_parser,
};

use crate::IRes;
use crate::NomErr;
type ParseErr = PettyParseError;

pub fn parse(input: &str) -> Result<Node, NomErr> {
    final_parser(map(nodes, Node::Group))(input.trim())
}
fn nodes(input: &str) -> IRes<Box<[Node]>> {
    map(many0(node), Vec::into_boxed_slice)(input)
}
#[inline]
fn node(input: &str) -> IRes {
    sp(err(
        alt((statement, map(block, Node::Group), terminated_expr)),
        PettyParseError::Node,
    ))(input)
}
fn terminated_expr(input: &str) -> IRes {
    err(
        alt((set_equals, node_value)).terminated(cut(spar(';'))),
        ParseErr::TermExpr,
    )(input)
}
fn set_equals(input: &str) -> IRes {
    map(
        separated_pair(sp(ident), spar('='), node_expr),
        |(ident, expr)| Node::SetEq(ident, Box::new(expr)),
    )(input)
}
#[inline]
fn node_expr(i: &str) -> IRes {
    err(sp(alt((bin_expr, node_value))), ParseErr::Expr)(i)
}
fn node_value(input: &str) -> IRes {
    alt((unary_expr, node_value_raw))(input)
}
fn node_value_raw(input: &str) -> IRes {
    alt((
        literal.map(Node::Literal),
        function_call,
        sp(ident).map(Node::Ident),
    ))(input)
}
fn unary_expr(input: &str) -> IRes {
    let unary_op = sp(alt((
        map(char('!'), |_| UnaryOp::Not),
        map(char('+'), |_| UnaryOp::Plus),
        map(char('-'), |_| UnaryOp::Neg),
    )));
    map(pair(unary_op, node_value), |(op, node)| {
        Node::UnaryOp(op, Box::new(node))
    })(input)
}
fn function_params(i: &str) -> IRes<Box<[String]>> {
    let (rem, nodes) = sp(delimited(
        spar('('),
        terminated(separated_list0(spar(','), sp(ident)), opt(spar(','))),
        spar(')'),
    ))(i)?;
    Ok((rem, nodes.into_boxed_slice()))
}
fn function_call(i: &str) -> IRes {
    pair(
        sp(ident),
        delimited(spar('('), function_args, cut(spar(')'))),
    )
    .map(|(ident, args)| Node::FuncCall(ident, args))
    .parse(i)
}
fn function_args(i: &str) -> IRes<Box<[Node]>> {
    let (rem, nodes) = sp(separated_list0(spar(','), sp(node_expr)))(i)?;
    Ok((rem, nodes.into_boxed_slice()))
}
fn block(i: &str) -> IRes<Box<[Node]>> {
    delimited(spar('{'), nodes, spar('}'))(i)
}
fn fold_exprs(initial: Node, remainder: Vec<(BinOp, Node)>) -> Node {
    remainder.into_iter().fold(initial, |acc, (op, expr)| {
        Node::BinExpr(op, Box::new((acc, expr)))
    })
}
fn eat_comments(i: &str) -> &str {
    let trimmed = i.trim_start();
    if !trimmed.starts_with("//") {
        return trimmed;
    }
    let end_of_comment = trimmed.find('\n').map_or(i.len(), |idx| idx + 1);
    &i[end_of_comment..]
}
fn ident(i: &str) -> IRes<String> {
    err(
        recognize(tuple((alt((alpha, char('_'))), take_while(is_ident_char)))),
        ParseErr::Ident,
    )
    .map(str::to_owned)
    .parse(i)
}
fn literal(i: &str) -> IRes<Literal> {
    sp(err(
        alt((
            map(boolean, Literal::Bool),
            map(float, Literal::Float),
            map(int, Literal::Int),
            map(string, |s| Literal::String(s.to_owned().into_boxed_str())),
            map(list, |vec| Literal::List(vec.into_boxed_slice())),
        )),
        ParseErr::Literal,
    ))(i)
}
#[inline]
fn is_ident_char(c: char) -> bool {
    matches!(c, 'a'..='z'|'A'..='Z'|'0'..='9'|'_')
}
fn list(i: &str) -> IRes<Vec<Node>> {
    delimited(
        char('['),
        separated_list0(spar(','), node_expr).terminated(opt(spar(','))),
        cut(spar(']')),
    )(i)
}
#[inline]
fn int(i: &str) -> IRes<i128> {
    map(digit1, |s: &str| s.parse().unwrap())(i)
}
fn float(i: &str) -> IRes<f64> {
    let mut parser = recognize(err(
        tuple((
            sp(opt(one_of("+-"))),
            sp(digit1),
            char('.'),
            cut(err(digit1, ParseErr::FloatDigit)),
        )),
        ParseErr::Float,
    ));
    let (rem, consumed) = match parser(i) {
        Ok(output) => output,
        Err(err) => return Err(err),
    };
    let float = consumed
        .parse()
        .unwrap_or_else(|e| panic!("{e} : {consumed:?}"));
    Ok((rem, float))
}
#[inline]
fn string(i: &str) -> IRes<&str> {
    delimited(char('"'), take_while(|c| c != '"'), cut(char('"')))(i)
}
#[inline]
fn boolean(input: &str) -> IRes<bool> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}
fn alpha(i: &str) -> IRes<char> {
    one_of("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}
fn sp<'a, O, E, P: Parser<&'a str, O, E>>(
    mut parser: P,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E> {
    move |i: &'a str| parser.parse(eat_comments(i))
}

fn err<'a, O, P: Parser<&'a str, O, NomErr<'a>>>(
    mut parser: P,
    msg: PettyParseError,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, NomErr<'a>> {
    move |i: &'a str| {
        parser.parse(i).map_err(|e| {
            let err = ErrorTree::Base {
                location: i,
                kind: nom_supreme::error::BaseErrorKind::External(Box::new(msg)),
            };
            match e {
                nom::Err::Error(_) => nom::Err::Error(err),
                nom::Err::Failure(e) => match &e {
                    ErrorTree::Base { location: _, kind }
                        if matches!(kind, BaseErrorKind::Expected(_)) =>
                    {
                        nom::Err::Failure(e)
                    }
                    _ => nom::Err::Failure(err),
                },
                nom::Err::Incomplete(_) => todo!(),
            }
        })
    }
}

#[inline]
fn spar<'a, E: nom::error::ParseError<&'a str>>(
    ch: char,
) -> impl FnMut(&'a str) -> IResult<&'a str, char, E> {
    move |i: &'a str| sp(char(ch))(i)
}
