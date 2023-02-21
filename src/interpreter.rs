use crate::ast::Node;

pub fn interpret(ast: Node) {
    Interpreter::init().execute(ast);
}

pub struct Interpreter {
}

impl Interpreter {
    pub fn init() -> Self {
        Self {}
    }
    pub fn execute(&mut self, node: Node) {
        match node {
            Node::Block(nodes) => todo!(),
            Node::BinExpr(op, nodes) => todo!(),
            Node::BreakState => todo!(),
            Node::ClassDef(name, fields, methods) => todo!(),
            Node::Empty => todo!(),
            Node::ForLoop(target, iter, block) => todo!(),
            Node::FuncCall(name, args) => todo!(),
            Node::FuncDef(name, params, block) => todo!(),
            Node::Ident(ident) => todo!(),
            Node::IfState(condition, block, or_else) => todo!(),
            Node::Literal(literal) => todo!(),
            Node::ReturnState(expr) => todo!(),
            Node::SetEq(var, expr) => todo!(),
            Node::UnaryOp(op, expr) => todo!(),
            Node::WhileLoop(condition, body) => todo!()
        }
    }
}
