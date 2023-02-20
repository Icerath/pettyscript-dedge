#[cfg(test)]
mod parser_tests {
    use super::super::{bin_expr, parse, BinOp, Node, UnaryOp};
    fn assert_expected(source: &str, expected: Vec<Node>) {
        let output = parse(source).unwrap();
        assert_eq!(output, Node::group(expected));
    }
    #[test]
    fn test_set_equals() {
        let source = "var = 10;";
        let expected = Node::set_eq("var", Node::literal(10));
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn test_set_equals_expr() {
        let source = "two_pi = pi() * 2;";
        let expected = Node::set_eq(
            "two_pi",
            Node::bin_expr(BinOp::Mul, Node::func_call("pi", vec![]), Node::literal(2)),
        );
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn test_bin_expr_order() {
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
    fn test_complex_expr() {
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
    fn test_simple_class() {
        let source = "class Point(x, y);";
        let expected = Node::class_def("Point", vec!["x", "y"], vec![]);
        assert_expected(source, vec![expected]);
    }
    #[test]
    fn test_class_methods() {
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
                vec![Node::ReturnState(Box::new(Node::func_call(
                    "Point",
                    vec![
                        Node::bin_expr(BinOp::Add, Node::ident("self.x"), Node::ident("other.x")),
                        Node::bin_expr(BinOp::Add, Node::ident("self.y"), Node::ident("other.y")),
                    ],
                )))],
            )],
        );
        assert_expected(source, vec![expected]);
    }
}
