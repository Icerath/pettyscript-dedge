#[cfg(test)]
mod parser_tests {
    use std::sync::Arc;

    use super::super::{bin_expr, parse, BinOp, Node, UnaryOp};
    fn assert_expected(source: &str, expected: Vec<Node>) {
        let output = parse(source).unwrap();
        assert_eq!(output, Node::Globals(expected.into()));
    }
    #[test]
    fn set_equals() {
        let source = "var = 10;";
        let expected = Node::set_eq("var", Node::literal(10));
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn set_equals_expr() {
        let source = "two_pi = pi() * 2;";
        let expected = Node::set_eq(
            "two_pi",
            Node::bin_expr(BinOp::Mul, Node::func_call("pi", vec![]), Node::literal(2)),
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn bin_expr_order() {
        let source = "1 + 2 - 3 * 4 / 5 > 10 && true";
        let expected = Node::bin_expr(
            BinOp::And,
            Node::bin_expr(
                BinOp::GT,
                Node::bin_expr(
                    BinOp::Sub,
                    Node::literal_expr(BinOp::Add, 1, 2),
                    Node::bin_expr(
                        BinOp::Div,
                        Node::literal_expr(BinOp::Mul, 3, 4),
                        Node::literal(5),
                    ),
                ),
                Node::literal(10),
            ),
            Node::literal(true),
        );
        let output = bin_expr(source).unwrap().1;
        assert_eq!(output, expected);
    }
    #[test]
    fn unary_expr() {
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
    fn line_comments() {
        let source = "//Hello!\none = 1;//Two\n//Comments\ntwo = 2;//End";
        let expected = vec![
            Node::set_eq("one", Node::literal(1)),
            Node::set_eq("two", Node::literal(2)),
        ];
        assert_expected(source, expected);
    }
    #[test]
    fn multiline_comments() {
        let source = "/*Hello*/i = 0;/*1\n2\n3*/";
        let expected = Node::set_eq("i", Node::literal(0));
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn complex_expr() {
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
    #[test]
    fn simple_function() {
        let source = r#"fn greet(name) { print("Hello " + name); }"#;
        let expected = Node::func_def(
            "greet",
            vec!["name"],
            vec![Node::func_call(
                "print",
                vec![Node::bin_expr(
                    BinOp::Add,
                    Node::literal("Hello "),
                    Node::ident("name"),
                )],
            )],
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn simple_class() {
        let source = "class Point(x, y);";
        let expected = Node::class_def("Point", vec!["x", "y"], vec![]);
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn class_methods() {
        let source = "
            class Point(x, y) {
                fn add(self, other) {
                    return Point(self.x + other.x, self.y + other.y);
                }
            }";
        let expected = Node::class_def(
            "Point",
            vec!["x", "y"],
            vec![Node::func_def(
                "add",
                vec!["self", "other"],
                vec![Node::ReturnState(Arc::new(Node::func_call(
                    "Point",
                    vec![
                        Node::bin_expr(
                            BinOp::Add,
                            Node::bin_expr(BinOp::GetItem, Node::ident("self"), Node::ident("x")),
                            Node::bin_expr(BinOp::GetItem, Node::ident("other"), Node::ident("x")),
                        ),
                        Node::bin_expr(
                            BinOp::Add,
                            Node::bin_expr(BinOp::GetItem, Node::ident("self"), Node::ident("y")),
                            Node::bin_expr(BinOp::GetItem, Node::ident("other"), Node::ident("y")),
                        ),
                    ],
                )))],
            )],
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn set_eq_type_hints() {
        let source = "num: int = 0;";
        let expected = Node::set_eq("num", Node::literal(0));
        assert_expected(source, vec![expected]);
        let source_err = "self.x: int = 0;";
        parse(source_err).unwrap_err();
    }
    #[test]
    fn function_type_hints() {
        let source = "fn squared(num: int) {
            return num * num;
        }";
        let expected = Node::func_def(
            "squared",
            vec!["num"],
            vec![Node::ReturnState(Arc::new(Node::bin_expr(
                BinOp::Mul,
                Node::ident("num"),
                Node::ident("num"),
            )))],
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn if_statement() {
        let source = "if true {}";
        let expected = Node::if_state(Node::literal(true), vec![], None);
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn if_chain() {
        let source = "if x { print(x); } elif y { print(y); } else { print(z); }";
        let expected = Node::if_state(
            Node::ident("x"),
            vec![Node::func_call("print", vec![Node::ident("x")])],
            Some(Node::if_state(
                Node::ident("y"),
                vec![Node::func_call("print", vec![Node::ident("y")])],
                Some(Node::block(vec![Node::func_call(
                    "print",
                    vec![Node::ident("z")],
                )])),
            )),
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn list_literal() {
        let source = "[1, 2, 3];";
        let expected = Node::literal(vec![Node::literal(1), Node::literal(2), Node::literal(3)]);
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn list_literal_exprs() {
        let source = r#"[1+2, pi(), "Hello, World!"];"#;
        let expected = Node::literal(vec![
            Node::bin_expr(BinOp::Add, Node::literal(1), Node::literal(2)),
            Node::func_call("pi", vec![]),
            Node::literal("Hello, World!"),
        ]);
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn get_item() {
        let source = "[1, 2, 3].len;";
        let expected = Node::bin_expr(
            BinOp::GetItem,
            Node::literal(vec![Node::literal(1), Node::literal(2), Node::literal(3)]),
            Node::ident("len"),
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn get_item_layered() {
        let source = "[1, 2, 3].len.abs().ans;";
        let expected = Node::bin_expr(
            BinOp::GetItem,
            Node::bin_expr(
                BinOp::GetItem,
                Node::bin_expr(
                    BinOp::GetItem,
                    Node::literal(vec![Node::literal(1), Node::literal(2), Node::literal(3)]),
                    Node::ident("len"),
                ),
                Node::func_call("abs", vec![]),
            ),
            Node::ident("ans"),
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn get_item_function() {
        let source = "1.max(2);";
        let expected = Node::bin_expr(
            BinOp::GetItem,
            Node::literal(1),
            Node::func_call("max", vec![Node::literal(2)]),
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn get_item_precedence() {
        let source = "1 + 1.abs();";
        let expected = Node::bin_expr(
            BinOp::Add,
            Node::literal(1),
            Node::bin_expr(
                BinOp::GetItem,
                Node::literal(1),
                Node::func_call("abs", vec![]),
            ),
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn closures() {
        let source = "|x, y| { return x + y; };";
        let expected = Node::closure(
            vec!["x", "y"],
            &[Node::ReturnState(Arc::new(Node::bin_expr(
                BinOp::Add,
                Node::ident("x"),
                Node::ident("y"),
            )))],
        );
        assert_expected(source, vec![expected]);
    }
}

#[cfg(test)]
mod failing_tests {
    use super::super::parse;
    #[test]
    fn fail_get_item() {
        let source = "1.1;";
        parse(source).unwrap();
    }
}
