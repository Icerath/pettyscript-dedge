#[allow(clippy::wildcard_imports)]
use super::*;

pub fn statement(input: &str) -> IRes {
    alt((
        if_statement,
        while_statement,
        for_loop,
        break_statement,
        return_statement,
        function_def,
        class_def,
    ))(input)
}
fn function_def(i: &str) -> IRes {
    preceded(
        keyword_name("fn"),
        cut(tuple((
            sp(ident),
            delimited(spar('('), params, spar(')')),
            block,
        ))),
    )
    .map(|(ident, params, block)| Node::FuncDef(ident, params, block))
    .parse(i)
}
fn class_def(input: &str) -> IRes {
    preceded(
        keyword_name("class"),
        cut(map(
            tuple((
                sp(strict_ident),
                delimited(spar('('), params, spar(')')),
                alt((
                    map(spar(';'), |_| Vec::new()),
                    delimited(spar('{'), many0(function_def), spar('}')),
                )),
            )),
            |(name, fields, functions)| Node::ClassDef(name, fields, functions.into_boxed_slice()),
        )),
    )(input)
}
fn if_statement(input: &str) -> IRes {
    if_segment(input)
}
fn if_segment(input: &str) -> IRes {
    preceded(
        keyword_name("if"),
        cut(tuple((node_expr, block, opt(or_else_segment)))),
    )
    .map(|(expr, block, or_else)| Node::IfState(Box::new(expr), block, or_else.map(Box::new)))
    .parse(input)
}
fn or_else_segment(input: &str) -> IRes {
    alt((
        preceded(
            keyword_name("elif"),
            cut(tuple((node_expr, block, opt(or_else_segment)))),
        )
        .map(|(expr, block, or_else)| Node::IfState(Box::new(expr), block, or_else.map(Box::new))),
        preceded(keyword_name("else"), map(block, Node::Block)),
    ))
    .parse(input)
}

fn while_statement(i: &str) -> IRes {
    preceded(keyword_name("while"), cut(pair(node_expr, block)))
        .map(|(n1, n2)| Node::WhileLoop(Box::new(n1), n2))
        .parse(i)
}
fn for_loop(i: &str) -> IRes {
    preceded(
        keyword_name("for"),
        cut(tuple((terminated(sp(ident), spar(':')), node_expr, block))),
    )
    .map(|(name, expr, block)| Node::ForLoop(name, Box::new(expr), block))
    .parse(i)
}
fn break_statement(i: &str) -> IRes {
    let (rem, _) = pair(keyword_name("break"), cut(spar(';')))(i)?;
    Ok((rem, Node::BreakState))
}
fn return_statement(i: &str) -> IRes {
    delimited(
        keyword_name("return"),
        opt(preceded(one_of(" \n"), node_expr)),
        cut(spar(';')),
    )
    .map(|node| Node::ReturnState(Box::new(node.unwrap_or(Node::Literal(Literal::Null)))))
    .parse(i)
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
