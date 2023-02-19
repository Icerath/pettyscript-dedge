mod bin_expr;
mod tests;
use bin_expr::bin_expr;

use crate::{
    ast::{BinOp, Literal, Node},
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
fn node(i: &str) -> IRes<Node> {
    sp(err(
        alt((statement, map(block, Node::Group), terminated_expr)),
        PettyParseError::Node,
    ))(i)
}

fn terminated_expr(i: &str) -> IRes<Node> {
    err(
        alt((set_equals, node_value)).terminated(cut(spar(';'))),
        ParseErr::TermExpr,
    )(i)
}
fn set_equals(i: &str) -> IRes {
    map(
        separated_pair(sp(ident), spar('='), node_expr),
        |(ident, expr)| Node::SetEq(ident, Box::new(expr)),
    )(i)
}
#[inline]
fn node_expr(i: &str) -> IRes<Node> {
    err(sp(alt((bin_expr, node_value))), ParseErr::Expr)(i)
}
fn node_value(i: &str) -> IRes<Node> {
    alt((
        literal.map(Node::Literal),
        function_call,
        sp(ident).map(Node::Ident),
    ))(i)
}
#[inline]
fn statement(i: &str) -> IRes<Node> {
    alt((
        if_statement,
        while_statement,
        for_loop,
        break_statement,
        return_statement,
        function_def,
    ))(i)
}
fn function_def(i: &str) -> IRes<Node> {
    preceded(
        keyword_name("fn"),
        cut(tuple((sp(ident), function_params, block))),
    )
    .map(|(ident, params, block)| Node::FuncDef(ident, params, block))
    .parse(i)
}
fn if_statement(i: &str) -> IRes<Node> {
    preceded(keyword_name("if"), cut(pair(node_expr, block)))
        .map(|(n1, n2)| Node::IfState(Box::new(n1), n2))
        .parse(i)
}
fn while_statement(i: &str) -> IRes<Node> {
    preceded(keyword_name("while"), cut(pair(node_expr, block)))
        .map(|(n1, n2)| Node::WhileLoop(Box::new(n1), n2))
        .parse(i)
}
fn for_loop(i: &str) -> IRes<Node> {
    preceded(
        keyword_name("for"),
        cut(tuple((terminated(sp(ident), spar(':')), node_expr, block))),
    )
    .map(|(name, expr, block)| Node::ForLoop(name, Box::new(expr), block))
    .parse(i)
}
fn break_statement(i: &str) -> IRes<Node> {
    let (rem, _) = pair(keyword_name("break"), cut(spar(';')))(i)?;
    Ok((rem, Node::BreakState))
}
fn return_statement(i: &str) -> IRes<Node> {
    delimited(
        keyword_name("return"),
        opt(preceded(one_of(" \n"), node_expr)),
        cut(spar(';')),
    )
    .map(|node| Node::ReturnState(Box::new(node.unwrap_or(Node::Literal(Literal::Null)))))
    .parse(i)
}
fn function_params(i: &str) -> IRes<Box<[String]>> {
    let (rem, nodes) = sp(delimited(
        spar('('),
        terminated(separated_list0(spar(','), sp(ident)), opt(spar(','))),
        spar(')'),
    ))(i)?;
    Ok((rem, nodes.into_boxed_slice()))
}
fn function_call(i: &str) -> IRes<Node> {
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

fn keyword_name<'a>(name: &'static str) -> impl FnMut(&'a str) -> IRes<&'a str> {
    move |i: &'a str| {
        let (rem, output) = sp(tag(name))(i)?;
        if rem.starts_with(is_ident_char) {
            Err(nom::Err::Error(nom::error::ParseError::from_error_kind(
                rem,
                nom::error::ErrorKind::AlphaNumeric,
            )))
        } else {
            Ok((rem, output))
        }
    }
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
