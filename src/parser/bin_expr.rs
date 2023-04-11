#[allow(clippy::wildcard_imports)]
use super::*;
pub fn bin_expr(input: &str) -> IRes {
    err(condition, ParseErr::Expr)(input)
}
fn paren_bin_expr(i: &str) -> IRes {
    delimited(spar('('), bin_expr, spar(')'))(i)
}
fn condition(input: &str) -> IRes {
    let (input, initial) = comparison(input)?;
    let (input, remainder) = many0(pair(binop_cond, comparison))(input)?;
    Ok((input, fold_exprs(initial, remainder)))
}
fn comparison(input: &str) -> IRes {
    let (input, initial) = lower(input)?;
    let (input, remainder) = many0(pair(binop_comp, lower))(input)?;
    Ok((input, fold_exprs(initial, remainder)))
}
fn lower(input: &str) -> IRes {
    let (input, initial) = upper(input)?;
    let (input, remainder) = many0(pair(binop_lower, upper))(input)?;
    Ok((input, fold_exprs(initial, remainder)))
}
fn upper(input: &str) -> IRes {
    let (input, initial) = get_item(input)?;
    let (input, remainder) = many0(pair(binop_upper, get_item))(input)?;
    Ok((input, fold_exprs(initial, remainder)))
}
fn get_item(input: &str) -> IRes {
    let (input, initial) = factor(input)?;
    let (input, remainder) =
        many0(pair(map(spar('.'), |_| BinOp::GetItem), get_item_suffix))(input)?;
    Ok((input, fold_exprs(initial, remainder)))
}
fn get_item_suffix(input: &str) -> IRes {
    alt((function_call, map(sp(ident), Node::Ident)))(input)
}
fn factor(input: &str) -> IRes {
    alt((paren_bin_expr, node_value, unary_expr))(input)
}

fn binop_lower(input: &str) -> IRes<BinOp> {
    sp(alt((
        map(char('+'), |_| BinOp::Add),
        map(char('-'), |_| BinOp::Sub),
    )))(input)
}
fn binop_upper(input: &str) -> IRes<BinOp> {
    sp(alt((
        map(char('*'), |_| BinOp::Mul),
        map(char('/'), |_| BinOp::Div),
        map(char('%'), |_| BinOp::Mod),
    )))(input)
}

fn binop_cond(input: &str) -> IRes<BinOp> {
    sp(alt((
        map(tag("&&"), |_| BinOp::And),
        map(tag("||"), |_| BinOp::Or),
    )))(input)
}
fn binop_comp(input: &str) -> IRes<BinOp> {
    sp(alt((
        map(tag("<="), |_| BinOp::LTEq),
        map(tag(">="), |_| BinOp::GTEq),
        map(tag("<"), |_| BinOp::LT),
        map(tag(">"), |_| BinOp::GT),
        map(tag("=="), |_| BinOp::IsEq),
        map(tag("!="), |_| BinOp::NotEq),
    )))(input)
}
