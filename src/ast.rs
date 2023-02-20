use std::fmt;

#[derive(PartialEq)]
pub enum Node {
    Literal(Literal),
    Group(Box<[Node]>),
    BinExpr(BinOp, Box<(Node, Node)>),
    UnaryOp(UnaryOp, Box<Node>),
    Ident(Box<str>),
    FuncCall(Box<str>, Box<[Node]>),
    IfState(Box<Node>, Box<[Node]>),
    WhileLoop(Box<Node>, Box<[Node]>),
    ForLoop(Box<str>, Box<Node>, Box<[Node]>),
    ReturnState(Box<Node>),
    BreakState,
    FuncDef(Box<str>, Box<[Box<str>]>, Box<[Node]>),
    StructDef(Box<str>, Box<[Box<str>]>, Box<[Node]>),
    Empty,
    SetEq(Box<str>, Box<Node>),
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    And,
    Or,
    LT,
    GT,
    LTEq,
    GTEq,
    IsEq,
    NotEq,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
    Plus,
}

#[derive(PartialEq)]
pub enum Literal {
    Int(i128),
    Float(f64),
    String(Box<str>),
    List(Box<[Node]>),
    Null,
    Bool(bool),
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),

            Self::LT => write!(f, "<"),
            Self::GT => write!(f, ">"),
            Self::LTEq => write!(f, "<="),
            Self::GTEq => write!(f, ">="),
            Self::IsEq => write!(f, "=="),
            Self::NotEq => write!(f, "!="),

            Self::And => write!(f, "&&"),
            Self::Or => write!(f, "||"),
        }
    }
}

impl fmt::Debug for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(int) => write!(f, "{int}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::String(string) => write!(f, "'{string}'"),
            Self::List(list) => f.debug_list().entries(list.iter()).finish(),
            Self::Null => write!(f, "null"),
            Self::Bool(bool) => write!(f, "{bool}"),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BinExpr(op, nodes) => f
                .debug_struct("expr")
                .field("left", &nodes.0)
                .field("op", op)
                .field("right", &nodes.1)
                .finish(),
            Self::UnaryOp(op, node) => f
                .debug_struct("unary_expr")
                .field("op", op)
                .field("inner", node)
                .finish(),
            Self::BreakState => write!(f, "break"),
            Self::Empty => write!(f, "empty"),
            Self::ForLoop(ident, expr, block) => f
                .debug_struct("for_loop")
                .field("target", ident)
                .field("iter", expr)
                .field("body", block)
                .finish(),
            Self::FuncCall(ident, args) => f
                .debug_struct("func")
                .field("name", ident)
                .field("args", args)
                .finish(),
            Self::FuncDef(ident, args, block) => f
                .debug_struct("func_def")
                .field("name", ident)
                .field("args", args)
                .field("body", block)
                .finish(),
            Self::Group(nodes) => f.debug_list().entries(nodes.iter()).finish(),
            Self::Ident(ident) => f.debug_tuple("Ident").field(ident).finish(),
            Self::IfState(expr, block) => f
                .debug_struct("if")
                .field("condition", expr)
                .field("body", block)
                .finish(),
            Self::Literal(literal) => write!(f, "{literal:?}"),
            Self::ReturnState(expr) => f.debug_tuple("return").field(expr).finish(),
            Self::SetEq(ident, expr) => f
                .debug_struct("set_eq")
                .field("left", ident)
                .field("right", expr)
                .finish(),
            Self::WhileLoop(condition, expr) => f
                .debug_struct("while")
                .field("condition", condition)
                .field("body", expr)
                .finish(),
            Self::StructDef(name, params, functions) => f
                .debug_struct("struct")
                .field("name", name)
                .field("params", params)
                .field("functions", functions)
                .finish(),
        }
    }
}
