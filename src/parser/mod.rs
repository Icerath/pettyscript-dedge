mod bin_expr;
mod statements;
mod tests;

use std::sync::Arc;

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

use self::statements::keyword_name;
type ParseErr = PettyParseError;

pub fn parse(input: &str) -> Result<Node, NomErr> {
    final_parser(map(nodes, Node::Globals))(input)
}
fn nodes(input: &str) -> IRes<Arc<[Node]>> {
    map(many0(node), Arc::from)(input)
}
#[inline]
fn node(input: &str) -> IRes {
    let (rem, output) = sp(err(
        alt((statement, map(block, Node::Block), terminated_expr)),
        PettyParseError::Node,
    ))(input)?;
    Ok((eat_comments(rem), output))
}
fn terminated_expr(input: &str) -> IRes {
    err(
        alt((set_equals, set_item_index, bin_expr)).terminated(cut(spar(';'))),
        ParseErr::TermExpr,
    )(input)
}
fn closure(input: &str) -> IRes {
    map(
        pair(delimited(spar('|'), params, spar('|')), block),
        |(args, body)| Node::Closure(args, body),
    )(input)
}

fn set_equals(input: &str) -> IRes {
    map(
        separated_pair(type_hinted, spar('='), node_expr),
        |(ident, expr)| Node::SetEq(ident, Arc::new(expr)),
    )(input)
}

fn set_item_index(input: &str) -> IRes {
    map(
        separated_pair(
            pair(sp(ident), delimited(spar('['), bin_expr, spar(']'))),
            spar('='),
            bin_expr,
        ),
        |((ident, index), expr)| Node::SetItemIndex(ident, index.into(), expr.into()),
    )(input)
}

#[inline]
fn node_expr(input: &str) -> IRes {
    bin_expr(input)
}
fn node_value(input: &str) -> IRes {
    alt((unary_expr, node_value_raw, closure))(input)
}
fn node_value_raw(input: &str) -> IRes {
    alt((
        literal.map(Node::Literal),
        function_call,
        get_item_index,
        sp(ident).map(Node::Ident),
    ))(input)
}
fn get_item_index(input: &str) -> IRes {
    map(
        pair(sp(ident), delimited(spar('['), bin_expr, spar(']'))),
        |(ident, expr)| Node::GetItemIndex(ident, expr.into()),
    )(input)
}

fn unary_expr(input: &str) -> IRes {
    let unary_op = sp(alt((
        map(char('!'), |_| UnaryOp::Not),
        map(char('+'), |_| UnaryOp::Plus),
        map(char('-'), |_| UnaryOp::Neg),
    )));
    map(
        pair(unary_op, alt((node_value, node_expr))),
        |(op, node)| Node::UnaryOp(op, Arc::new(node)),
    )(input)
}
fn params(input: &str) -> IRes<Arc<[Arc<str>]>> {
    let (rem, nodes) = terminated(separated_list0(spar(','), type_hinted), opt(spar(',')))(input)?;
    Ok((rem, nodes.into()))
}
fn function_call(i: &str) -> IRes {
    pair(
        sp(ident),
        delimited(spar('('), function_args, cut(spar(')'))),
    )
    .map(|(name, args)| Node::FuncCall(name, args))
    .parse(i)
}
fn function_args(i: &str) -> IRes<Arc<[Node]>> {
    let (rem, nodes) = sp(separated_list0(spar(','), sp(node_expr)))(i)?;
    Ok((rem, nodes.into()))
}
fn block(i: &str) -> IRes<Arc<[Node]>> {
    delimited(spar('{'), nodes, spar('}'))(i)
}
fn fold_exprs(initial: Node, remainder: Vec<(BinOp, Node)>) -> Node {
    remainder.into_iter().fold(initial, |acc, (op, expr)| {
        Node::BinExpr(op, Arc::new((acc, expr)))
    })
}
fn eat_comments(mut input: &str) -> &str {
    input = input.trim();
    loop {
        let end_of_comment = if input.starts_with("//") {
            input.find('\n').map(|idx| idx + 1) // 1 = '\n';
        } else if let Some(suffix) = input.strip_prefix("/*") {
            suffix.find("*/").map(|idx| idx + 4) // 4 = prefix.len() + suffix.len();
        } else {
            break input;
        };
        let end_of_comment = end_of_comment.unwrap_or(input.len());
        input = &input[end_of_comment..];
        input = input.trim();
    }
}
fn type_hinted(input: &str) -> IRes<Arc<str>> {
    alt((
        terminated(sp(ident), opt(pair(spar(':'), sp(ident)))),
        sp(ident),
    ))(input)
}
fn ident(i: &str) -> IRes<Arc<str>> {
    err(
        recognize(tuple((alt((alpha, char('_'))), take_while(is_ident_char)))),
        ParseErr::Ident,
    )
    .map(Arc::from)
    .parse(i)
}
fn literal(i: &str) -> IRes<Literal> {
    sp(err(
        alt((
            map(boolean, Literal::Bool),
            map(float, Literal::Float),
            map(int, Literal::Int),
            map(string, |s| Literal::String(s.into())),
            map(list, |vec| Literal::List(vec.into())),
            map(keyword_name("null"), |_| Literal::Null),
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
            err(digit1, ParseErr::FloatDigit),
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
            let err = new_error(i, msg);
            match e {
                nom::Err::Error(_) => nom::Err::Error(err),
                nom::Err::Failure(e) => match &e {
                    ErrorTree::Base { location: _, kind }
                        if matches!(kind, BaseErrorKind::External(_err)) =>
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

fn new_error(i: &str, kind: ParseErr) -> NomErr {
    ErrorTree::Base {
        location: i,
        kind: nom_supreme::error::BaseErrorKind::External(Box::new(kind)),
    }
}

#[inline]
fn spar<'a, E: nom::error::ParseError<&'a str>>(
    ch: char,
) -> impl FnMut(&'a str) -> IResult<&'a str, char, E> {
    move |i: &'a str| sp(char(ch))(i)
}
