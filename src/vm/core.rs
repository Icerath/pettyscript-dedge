use crate::ast::{BinOp, Node};

use super::{
    builtins,
    field_dict::FieldDict,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
};

pub type Vm = VirtualMachine;

pub struct VirtualMachine {
    fields: FieldDict,
    null: PettyObject,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            fields: FieldDict::new(),
            null: builtins::PtyNull.into(),
        }
    }
    pub fn null(&self) -> PettyObject {
        self.null.clone()
    }
    pub fn load_builtin<PtyObj: PettyObjectType + 'static>(&mut self, name: &str, object: PtyObj) {
        self.fields.write(name, object.into());
    }
}

impl VirtualMachine {
    pub fn evaluate(&mut self, node: &Node) -> PettyObject {
        match node {
            Node::Globals(nodes) | Node::Block(nodes) => self.execute_nodes(nodes),
            Node::SetEq(name, expr) => self.set_eq(name, expr),
            Node::BinExpr(op, nodes) => return self.bin_expr(*op, &nodes.0, &nodes.1),
            Node::Literal(literal) => return builtins::create_literal(literal),
            Node::FuncCall(name, args) => return self.func_call(name, args),
            _ => todo!("{node:?}"),
        };
        self.null()
    }
    pub fn execute_nodes(&mut self, nodes: &[Node]) {
        for node in nodes {
            self.evaluate(node);
        }
    }
    pub fn set_eq(&mut self, name: &str, expr: &Node) {
        let value = self.evaluate(expr);
        println!("{name}: {}", &value);
        self.fields[name] = value;
    }
    pub fn bin_expr(&mut self, op: BinOp, lhs: &Node, rhs: &Node) -> PettyObject {
        let lhs = self.evaluate(lhs);
        let rhs = self.evaluate(rhs);
        let function_name = op_to_function(op);
        let function = lhs.get_item(self, lhs.clone(), function_name);
        let args = FuncArgs(vec![lhs, rhs]);
        function.call(self, function.clone(), args)
    }
    pub fn func_call(&mut self, name: &str, args: &[Node]) -> PettyObject {
        let args: Vec<_> = args.iter().map(|arg| self.evaluate(arg)).collect();
        let function = self.fields.read(name);
        function.call(self, function.clone(), FuncArgs(args))
    }
}

#[must_use]
#[inline]
fn op_to_function(op: BinOp) -> &'static str {
    match op {
        BinOp::Add => "__add__",
        BinOp::Sub => "__sub__",
        BinOp::Mul => "__mul__",
        BinOp::Div => "__div__",
        _ => todo!(),
    }
}
