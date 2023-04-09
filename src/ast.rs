use std::{fmt, sync::Arc};

#[derive(PartialEq, Clone)]
pub enum Node {
    Literal(Literal),
    Block(Arc<[Node]>),
    Globals(Arc<[Node]>),
    Closure(Arc<[Arc<str>]>, Arc<[Node]>),
    BinExpr(BinOp, Arc<(Node, Node)>),
    UnaryOp(UnaryOp, Arc<Node>),
    Ident(Arc<str>),
    FuncCall(Arc<str>, Arc<[Node]>),
    IfState(Arc<Node>, Arc<[Node]>, Option<Arc<Node>>),
    WhileLoop(Arc<Node>, Arc<[Node]>),
    ForLoop(Arc<str>, Arc<Node>, Arc<[Node]>),
    ReturnState(Arc<Node>),
    BreakState,
    FuncDef(Arc<str>, Arc<[Arc<str>]>, Arc<[Node]>),
    ClassDef(Arc<str>, Arc<[Arc<str>]>, Arc<[Node]>),
    Empty,
    SetEq(Arc<str>, Arc<Node>),
    GetItemIndex(Arc<str>, Arc<Node>),
    SetItemIndex(Arc<str>, Arc<Node>, Arc<Node>),
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

    GetItem,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Not,
    Neg,
    Plus,
}

#[derive(PartialEq, Clone)]
pub enum Literal {
    Int(i128),
    Float(f64),
    String(Arc<str>),
    List(Arc<[Node]>),
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
            Self::GetItem => write!(f, "."),
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
            Self::Closure(args, body) => f
                .debug_struct("closure")
                .field("args", args)
                .field("body", body)
                .finish(),
            Self::BinExpr(op, nodes) => f
                .debug_struct("expr")
                .field("op", op)
                .field("left", &nodes.0)
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
            Self::FuncCall(name, args) => f
                .debug_struct("func_call")
                .field("name", name)
                .field("args", args)
                .finish(),
            Self::FuncDef(ident, args, block) => f
                .debug_struct("func_def")
                .field("name", ident)
                .field("args", args)
                .field("body", block)
                .finish(),
            Self::Block(nodes) | Self::Globals(nodes) => {
                f.debug_list().entries(nodes.iter()).finish()
            }
            Self::Ident(ident) => f.debug_tuple("Ident").field(ident).finish(),
            Self::IfState(expr, block, or_else) => f
                .debug_struct("if")
                .field("condition", expr)
                .field("body", block)
                .field("or_else", &or_else.iter().collect::<Vec<_>>())
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
            Self::ClassDef(name, fields, functions) => f
                .debug_struct("class")
                .field("name", name)
                .field("fields", fields)
                .field("functions", functions)
                .finish(),
            Self::GetItemIndex(ident, expr) => f
                .debug_struct("get_index")
                .field("ident", ident)
                .field("expr", expr)
                .finish(),
            Self::SetItemIndex(ident, index, expr) => f
                .debug_struct("set_index")
                .field("ident", ident)
                .field("index", index)
                .field("expr", expr)
                .finish(),
        }
    }
}

impl Node {
    pub fn ident(string: &str) -> Self {
        Self::Ident(string.into())
    }
    pub fn unary_expr(op: UnaryOp, node: Node) -> Self {
        Self::UnaryOp(op, Arc::new(node))
    }
    pub fn literal_expr<L: Into<Literal>, R: Into<Literal>>(op: BinOp, left: L, right: R) -> Self {
        Self::bin_expr(op, Node::literal(left), Node::literal(right))
    }
    pub fn bin_expr(op: BinOp, left: Node, right: Node) -> Self {
        Node::BinExpr(op, Arc::new((left, right)))
    }
    pub fn func_call(name: &str, args: Vec<Node>) -> Self {
        Self::FuncCall(name.into(), args.into())
    }
    pub fn literal(literal: impl Into<Literal>) -> Self {
        Self::Literal(literal.into())
    }
    pub fn block(nodes: Vec<Node>) -> Self {
        Self::Block(nodes.into())
    }
    pub fn set_eq(ident: &str, value: Node) -> Self {
        Self::SetEq(ident.into(), Arc::new(value))
    }
    pub fn class_def(name: &str, fields: Vec<&str>, methods: Vec<Node>) -> Self {
        Self::ClassDef(name.into(), vec_box_str(fields), methods.into())
    }
    pub fn func_def(name: &str, params: Vec<&str>, block: Vec<Node>) -> Self {
        Self::FuncDef(name.into(), vec_box_str(params), block.into())
    }
    pub fn if_state(condition: Node, block: Vec<Node>, or_else: Option<Node>) -> Self {
        Self::IfState(Arc::new(condition), block.into(), or_else.map(Arc::new))
    }
    pub fn while_loop(condition: Node, block: Vec<Node>) -> Self {
        Self::WhileLoop(Arc::new(condition), block.into())
    }
    pub fn for_loop(target: &str, iter: Node, block: Vec<Node>) -> Self {
        Self::ForLoop(target.into(), iter.into(), block.into())
    }
    pub fn closure(args: Vec<&str>, body: &[Node]) -> Self {
        Self::Closure(vec_box_str(args), body.into())
    }
}
fn vec_box_str(input: Vec<&str>) -> Arc<[Arc<str>]> {
    input
        .into_iter()
        .map(std::convert::Into::into)
        .collect::<Vec<_>>()
        .into()
}

impl From<Literal> for Node {
    fn from(value: Literal) -> Self {
        Self::Literal(value)
    }
}

impl From<i128> for Literal {
    fn from(value: i128) -> Self {
        Self::Int(value)
    }
}
impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        Self::String(value.into())
    }
}
impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec<Node>> for Literal {
    fn from(value: Vec<Node>) -> Self {
        Self::List(value.into())
    }
}
