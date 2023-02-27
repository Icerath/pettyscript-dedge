use crate::ast::{BinOp, Node, UnaryOp};

use super::{
    builtins::{self, PtyBool, PtyNull},
    field_dict::FieldDict,
    function_args::FuncArgs,
    object::{PettyObject, PettyObjectType},
    petty_function::PettyFunction,
};

pub type Vm = VirtualMachine;

pub struct VirtualMachine {
    pub fields: FieldDict,
    pub return_val: Option<PettyObject>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            fields: FieldDict::default(),
            return_val: None,
        }
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
            Node::BinExpr(op, nodes) if *op == BinOp::GetItem => {
                return self.get_item(&nodes.0, &nodes.1)
            }
            Node::BinExpr(op, nodes) => return self.bin_expr(*op, &nodes.0, &nodes.1),
            Node::Literal(literal) => return builtins::create_literal(literal),
            Node::Ident(ident) => return self.fields.read(ident),
            Node::FuncCall(name, args) => return self.func_call(name, args),
            Node::FuncDef(name, args, block) => self.func_def(name, args, block),
            Node::ReturnState(expr) => self.return_val = Some(self.evaluate(expr)),
            Node::UnaryOp(op, expr) => return self.unary_expr(*op, expr),
            Node::IfState(condition, block, or_else) => {
                self.if_statement(condition, block, or_else.as_ref().map(Box::as_ref));
            }
            Node::WhileLoop(condition, block) => self.while_loop(condition, block),
            Node::ForLoop(target, iter, block) => self.for_loop(target, iter, block),
            _ => todo!("{node:?}"),
        };
        PtyNull.into()
    }
    pub fn execute_nodes(&mut self, nodes: &[Node]) {
        for node in nodes {
            self.evaluate(node);
        }
    }
    pub fn set_eq(&mut self, name: &str, expr: &Node) {
        let value = self.evaluate(expr);
        self.fields.write(name, value);
    }
    pub fn get_item(&mut self, left: &Node, right: &Node) -> PettyObject {
        let left = self.evaluate(left);
        let (function, args) = match right {
            Node::Ident(ident) => return left.get_item(self, left.clone(), ident),
            Node::FuncCall(name, args) => (left.get_item(self, left.clone(), name), args),
            _ => unreachable!(),
        };
        let mut args = self.evaluate_list(args);
        args.push(left);
        function.call(self, function.clone(), FuncArgs(args))
    }
    pub fn bin_expr(&mut self, op: BinOp, lhs: &Node, rhs: &Node) -> PettyObject {
        let lhs = self.evaluate(lhs);
        let rhs = self.evaluate(rhs);
        let function_name = op.into_petty_function();
        let function = lhs.get_item(self, lhs.clone(), function_name);
        let args = FuncArgs(vec![lhs, rhs]);
        function.call(self, function.clone(), args)
    }
    pub fn unary_expr(&mut self, op: UnaryOp, expr: &Node) -> PettyObject {
        let inner = self.evaluate(expr);
        let function_name = op.into_petty_function();
        let function = inner.get_item(self, inner.clone(), function_name);
        let args = FuncArgs(vec![inner]);
        function.call(self, function.clone(), args)
    }
    pub fn func_call(&mut self, name: &str, args: &[Node]) -> PettyObject {
        let args = self.evaluate_list(args);
        let function = self.fields.read(name);
        function.call(self, function.clone(), FuncArgs(args))
    }
    pub fn evaluate_list(&mut self, items: &[Node]) -> Vec<PettyObject> {
        items.iter().map(|arg| self.evaluate(arg)).collect()
    }
    #[allow(clippy::borrowed_box)]
    pub fn func_def(&mut self, name: &str, args: &Box<[Box<str>]>, block: &Box<[Node]>) {
        let function = PettyFunction::new(args.clone(), block.clone());
        self.fields.write(name, function.into());
    }
    pub fn if_statement(&mut self, condition: &Node, block: &[Node], or_else: Option<&Node>) {
        let condition = self.evaluate(condition);
        let condition = condition.call_method(self, "__bool__", FuncArgs(vec![]));
        let condition = condition
            .as_any()
            .downcast_ref::<PtyBool>()
            .expect("Expected bool");
        if condition.0 {
            return self.execute_nodes(block);
        }
        if let Some(node) = or_else {
            self.evaluate(node);
        };
    }
    pub fn while_loop(&mut self, condition: &Node, block: &[Node]) {
        while {
            let condition = self.evaluate(condition);
            let condition = condition.call_method(self, "__bool__", FuncArgs(vec![]));
            let condition = condition
                .as_any()
                .downcast_ref::<PtyBool>()
                .expect("Expected bool");
            condition.0
        } {
            self.execute_nodes(block);
        }
    }
    pub fn for_loop(&mut self, target: &str, iter: &Node, block: &[Node]) {
        todo!()
    }
}

impl BinOp {
    #[must_use]
    #[inline]
    pub fn into_petty_function(self) -> &'static str {
        match self {
            Self::Add => "__add__",
            Self::Sub => "__sub__",
            Self::Mul => "__mul__",
            Self::Div => "__div__",
            Self::IsEq => "__is_eq__",
            Self::LT => "__lt__",
            Self::GT => "__gt__",
            Self::LTEq => "__lt_eq__",
            Self::GTEq => "__gt_eq__",
            _ => todo!(),
        }
    }
}

impl UnaryOp {
    #[must_use]
    #[inline]
    pub fn into_petty_function(self) -> &'static str {
        match self {
            Self::Neg => "__neg__",
            Self::Not => "__not__",
            Self::Plus => panic!("Idk what to use this for yet"),
        }
    }
}
