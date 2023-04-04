use super::{
    builtins::{self, PtyBool, PtyNull, NULL},
    field_dict::FieldDict,
    function_args::FuncArgs,
    object::PettyObject,
    petty_class::PettyClass,
    petty_function::PettyFunction,
    preallocated::PreAllocated,
};
use crate::{
    ast::{BinOp, Literal, Node, UnaryOp},
    slim_rc::Rc,
};
pub type Vm = VirtualMachine;

#[derive(Default)]
pub struct VirtualMachine {
    pub preallocated: PreAllocated,
    pub fields: FieldDict,
    pub return_val: Option<PettyObject>,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn load_builtin(&mut self, name: &str, object: PettyObject) {
        self.fields.write(name.into(), object);
    }
}

impl VirtualMachine {
    pub fn evaluate(&mut self, node: &Node) -> PettyObject {
        match node {
            Node::Globals(nodes) | Node::Block(nodes) => self.execute_nodes(nodes),
            Node::SetEq(name, expr) => self.set_eq(name.clone(), expr),
            Node::BinExpr(op, nodes) if *op == BinOp::GetItem => {
                return self.get_item(&nodes.0, &nodes.1)
            }
            Node::BinExpr(op, nodes) => return self.bin_expr(*op, &nodes.0, &nodes.1),
            Node::Literal(literal) => {
                if let Literal::Int(int @ 0..=255) = literal {
                    return self.preallocated.get(*int as usize).unwrap();
                }
                return builtins::create_literal(literal);
            }
            Node::Ident(ident) => return self.fields.read(ident),
            Node::FuncCall(name, args) => return self.func_call(name, args),
            Node::FuncDef(name, args, block) => self.func_def(name.clone(), args, block),
            Node::ReturnState(expr) => self.return_val = Some(self.evaluate(expr)),
            Node::UnaryOp(op, expr) => return self.unary_expr(*op, expr),
            Node::IfState(condition, block, or_else) => {
                self.if_statement(condition, block, or_else.as_ref().map(Rc::as_ref));
            }
            Node::WhileLoop(condition, block) => self.while_loop(condition, block),
            Node::ForLoop(target, iter, block) => self.for_loop(target, iter, block),
            Node::ClassDef(name, fields, methods) => {
                self.class_def(name.clone(), fields.clone(), methods);
            }
            _ => todo!("{node:?}"),
        };
        NULL.clone()
    }
    pub fn execute_nodes(&mut self, nodes: &[Node]) {
        for node in nodes {
            if self.return_val.is_some() {
                break;
            }
            self.evaluate(node);
        }
    }
    pub fn set_eq(&mut self, name: Rc<str>, expr: &Node) {
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
        let mut items = Vec::with_capacity(args.len());
        items.push(left);
        for arg in args.iter() {
            items.push(self.evaluate(arg));
        }
        function.call(self, function.clone(), FuncArgs(items))
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
    pub fn func_call(&mut self, name: &Rc<str>, args: &[Node]) -> PettyObject {
        let args = self.evaluate_list(args);
        let function = self.fields.read(name);
        function.call(self, function.clone(), FuncArgs(args))
    }
    pub fn evaluate_list(&mut self, items: &[Node]) -> Vec<PettyObject> {
        items.iter().map(|arg| self.evaluate(arg)).collect()
    }
    pub fn func_def(&mut self, name: Rc<str>, args: &Rc<[Rc<str>]>, block: &Rc<[Node]>) {
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
        while self.return_val.is_none() && {
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
    pub fn class_def(&mut self, name: Rc<str>, fields: Rc<[Rc<str>]>, methods: &Rc<[Node]>) {
        let class = PettyClass::new(fields, methods.clone());
        self.fields.write(name, class.into());
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
            Self::Mod => "__mod__",
            Self::And => "__and__",
            Self::Or => "__or__",
            _ => todo!("{self}"),
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
