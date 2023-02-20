#[cfg(test)]
mod parser_tests {
    use crate::ast::UnaryOp;

    use super::super::{bin_expr, parse, BinOp, Literal, Node};
    fn assert_expected(source: &str, expected: Vec<Node>) {
        let output = parse(source).unwrap();
        assert_eq!(output, Node::group(expected));
    }
    #[test]
    fn test_set_equals() {
        let source = "var = 10;";
        let expected = r#"[set_eq { left: "var", right: 10 }]"#;
        let output = parse(source).unwrap();
        assert_eq!(format!("{output:?}"), expected);
    }
    #[test]
    fn test_set_equals_expr() {
        let source = "two_pi = pi() * 2;";
        let expected = concat!(
            r#"[set_eq { left: "two_pi", right: expr { left: "#,
            r#"func { name: "pi", args: [] }, op: Mul, right: 2 } }]"#
        );
        let output = parse(source).unwrap();
        assert_eq!(format!("{output:?}"), expected);
    }
    #[test]
    fn test_bin_expr_order() {
        let source = "1 + 2 - 3 * 4 / 5 > 10 && true";
        let expected = concat!(
            r#"expr { left: expr { left: expr { left: expr { left: expr { left: "#,
            r#"expr { left: 1, op: Add, right: 2 }, op: Sub, right: 3 }, op: "#,
            r#"Mul, right: 4 }, op: Div, right: 5 }, op: GT, right: 10 }, op: "#,
            r#"And, right: true }"#,
        );
        let output = bin_expr(source).unwrap().1;
        assert_eq!(format!("{output:?}"), expected);
    }
    #[test]
    fn test_unary_expr() {
        let source = "--1 + !i";
        let expected = Node::bin_expr(
            BinOp::Add,
            Node::unary_expr(
                UnaryOp::Neg,
                Node::unary_expr(UnaryOp::Neg, Node::literal(1)),
            ),
            Node::unary_expr(UnaryOp::Not, Node::ident("i")),
        );
        let output = bin_expr(source).unwrap().1;
        assert_eq!(output, expected);
    }
    #[test]
    fn test_line_comments() {
        let source = "//Hello!\none = 1;//Two\n//Comments\ntwo = 2;//End";
        let expected = vec![
            Node::set_eq("one", Node::literal(1)),
            Node::set_eq("two", Node::literal(2)),
        ];
        assert_expected(source, expected);
    }
    #[test]
    fn test_multiline_comments() {
        let source = "/*Hello*/i = 0;/*1\n2\n3*/";
        let expected = Node::set_eq("i", Node::literal(0));
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn test_direct_ast() {
        let source = "condition = 1 - get_pi() < 10 / 3 && get_pi() == 3.141516;";
        let expected = Node::set_eq(
            "condition",
            Node::bin_expr(
                BinOp::And,
                Node::bin_expr(
                    BinOp::LT,
                    Node::bin_expr(
                        BinOp::Sub,
                        Node::literal(1),
                        Node::func_call("get_pi", vec![]),
                    ),
                    Node::bin_expr(BinOp::Div, Node::literal(10), Node::literal(3)),
                ),
                Node::bin_expr(
                    BinOp::IsEq,
                    Node::func_call("get_pi", vec![]),
                    Node::literal(3.141_516),
                ),
            ),
        );
        assert_expected(source, vec![expected]);
    }
}
