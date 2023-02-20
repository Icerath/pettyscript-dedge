#[cfg(test)]
mod parser_tests {
    use super::super::{bin_expr, parse, BinOp, Literal, Node};
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
        let expected = concat!(
            r#"expr { left: unary_expr { op: Neg, inner: unary_expr { op: "#,
            r#"Neg, inner: 1 } }, op: Add, right: unary_expr { op: Not, inner: Ident("i") } }"#
        );
        let output = bin_expr(source).unwrap().1;
        assert_eq!(format!("{output:?}"), expected);
    }
    #[test]
    fn test_line_comments() {
        let source = "//Hello!\none = 1;//Two\n//Comments\ntwo = 2;//End";
        let expected = r#"[set_eq { left: "one", right: 1 }, set_eq { left: "two", right: 2 }]"#;
        let output = parse(source).unwrap();
        assert_eq!(format!("{output:?}"), expected);
    }
    #[test]
    fn test_multiline_comments() {
        let source = "/*Hello*/i = 0;/*1\n2\n3*/";
        let expected = r#"[set_eq { left: "i", right: 0 }]"#;
        let output = parse(source).unwrap();
        assert_eq!(format!("{output:?}"), expected);
    }
    #[test]
    fn test_direct_ast() {
        let source = "condition = 1 - get_pi() < 10 / 3 && get_pi() == 3.141516;";
        let output = parse(source).unwrap();
        let expected = Node::Group(Box::new([Node::SetEq(
            "condition".to_owned().into_boxed_str(),
            Box::new(Node::BinExpr(
                BinOp::And,
                Box::new((
                    Node::BinExpr(
                        BinOp::LT,
                        Box::new((
                            Node::BinExpr(
                                BinOp::Sub,
                                Box::new((
                                    Node::Literal(Literal::Int(1)),
                                    Node::FuncCall("get_pi".into(), Box::new([])),
                                )),
                            ),
                            Node::BinExpr(
                                BinOp::Div,
                                Box::new((
                                    Node::Literal(Literal::Int(10)),
                                    Node::Literal(Literal::Int(3)),
                                )),
                            ),
                        )),
                    ),
                    Node::BinExpr(
                        BinOp::IsEq,
                        Box::new((
                            Node::FuncCall("get_pi".into(), Box::new([])),
                            Node::Literal(Literal::Float(3.141_516)),
                        )),
                    ),
                )),
            )),
        )]));
        assert_eq!(output, expected);
    }
}
