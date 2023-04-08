use std::{
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard},
};

use super::{
    builtins::{PtyBool, PtyList, PtyNum, PtyStr, NULL},
    field_dict::FieldDict,
    function_args::FuncArgs,
    object::PettyObject,
    petty_class::PettyClass,
    petty_function::PettyFunction,
    preallocated::PreAllocated,
};
use crate::{
    ast::{BinOp, Literal, Node, UnaryOp},
    vm::builtins::PtyOption,
};
#[derive(Default)]
pub struct VirtualMachine {
    pub preallocated: PreAllocated,
    pub fields: FieldDict,
    pub return_val: Option<PettyObject>,
}
#[derive(Default, Clone)]
pub struct Vm(Arc<Mutex<VirtualMachine>>);

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn load_builtin(&self, name: &str, object: PettyObject) {
        self.lock().unwrap().fields.write(name.into(), object);
    }
    pub fn get(&self) -> MutexGuard<VirtualMachine> {
        let a = self.lock().unwrap();
        a
    }
}

impl Vm {
    pub fn evaluate(&self, node: &Node) -> PettyObject {
        match node {
            Node::Globals(nodes) | Node::Block(nodes) => self.execute_nodes(nodes),
            Node::SetEq(name, expr) => self.set_eq(name.clone(), expr),
            Node::BinExpr(op, nodes) if *op == BinOp::GetItem => {
                return self.get_item(&nodes.0, &nodes.1)
            }
            Node::BinExpr(op, nodes) => return self.bin_expr(*op, &nodes.0, &nodes.1),
            Node::Literal(literal) => {
                return self.create_literal(literal);
            }
            Node::Ident(ident) => return self.get().fields.read(ident),
            Node::FuncCall(name, args) => return self.func_call(name, args),
            Node::FuncDef(name, args, block) => self.func_def(name.clone(), args, block),
            Node::ReturnState(expr) => self.get().return_val = Some(self.evaluate(expr)),
            Node::UnaryOp(op, expr) => return self.unary_expr(*op, expr),
            Node::IfState(condition, block, or_else) => {
                self.if_statement(condition, block, or_else.as_ref().map(Arc::as_ref));
            }
            Node::WhileLoop(condition, block) => self.while_loop(condition, block),
            Node::ForLoop(target, iter, block) => self.for_loop(target, iter, block),
            Node::ClassDef(name, fields, methods) => {
                self.class_def(name.clone(), fields.clone(), methods);
            }
            Node::Closure(params, body) => return self.closure(params, body),
            _ => todo!("{node:?}"),
        };
        NULL.clone()
    }
    pub fn execute_nodes(&self, nodes: &[Node]) {
        for node in nodes {
            if self.get().return_val.is_some() {
                break;
            }
            self.evaluate(node);
        }
    }
    pub fn set_eq(&self, name: Arc<str>, expr: &Node) {
        let value = self.evaluate(expr);
        self.get().fields.write(name, value);
    }
    pub fn get_item(&self, left: &Node, right: &Node) -> PettyObject {
        let left = self.evaluate(left);

        let (function, args) = match right {
            Node::Ident(ident) => return left.get_item(self, &left, ident),
            Node::FuncCall(name, args) => (left.get_item(self, &left, name), args),
            _ => unreachable!(),
        };

        let mut items = Vec::with_capacity(args.len());
        items.push(left);
        items.extend(args.iter().map(|node| self.evaluate(node)));

        function.call(
            &self,
            &function,
            FuncArgs(items.iter().collect::<Vec<_>>().as_slice()),
        )
    }
    pub fn bin_expr(&self, op: BinOp, lhs: &Node, rhs: &Node) -> PettyObject {
        let lhs = self.evaluate(lhs);
        let rhs = self.evaluate(rhs);
        let function_name = op.into_petty_function();
        let function = lhs.get_item(self, &lhs, function_name);
        let binding = [&lhs, &rhs];
        let args = FuncArgs(&binding);
        function.call(self, &function, args)
    }
    pub fn unary_expr(&self, op: UnaryOp, expr: &Node) -> PettyObject {
        let inner = self.evaluate(expr);
        let function_name = op.into_petty_function();
        let function = inner.get_item(self, &inner, function_name);
        let binding = [&inner];
        let args = FuncArgs(&binding);
        function.call(self, &function, args)
    }
    pub fn func_call(&self, name: &Arc<str>, args: &[Node]) -> PettyObject {
        let args = self.evaluate_list(args);
        let function = self.get().fields.read(name);
        function.call(
            self,
            &function,
            FuncArgs(args.iter().collect::<Vec<_>>().as_slice()),
        )
    }
    pub fn evaluate_list(&self, items: &[Node]) -> Vec<PettyObject> {
        items.iter().map(|arg| self.evaluate(arg)).collect()
    }
    pub fn func_def(&self, name: Arc<str>, args: &Arc<[Arc<str>]>, block: &Arc<[Node]>) {
        let function = self.closure(args, block);
        self.get().fields.write(name, function);
    }
    pub fn closure(&self, args: &Arc<[Arc<str>]>, block: &Arc<[Node]>) -> PettyObject {
        PettyFunction::new(
            args.clone(),
            block.clone(),
            self.get().fields.scopes.clone(),
        )
        .into()
    }
    pub fn if_statement(&self, condition: &Node, block: &[Node], or_else: Option<&Node>) {
        let condition = self.evaluate(condition);
        let condition = condition.call_method(self, "__bool__", FuncArgs(&[]));
        let condition = condition.downcast_ref::<PtyBool>().expect("Expected Bool");
        if condition.0 {
            return self.execute_nodes(block);
        }
        if let Some(node) = or_else {
            self.evaluate(node);
        };
    }
    pub fn while_loop(&self, condition: &Node, block: &[Node]) {
        while self.get().return_val.is_none() && {
            let condition = self.evaluate(condition);
            let condition = condition.call_method(self, "__bool__", FuncArgs(&[]));
            let condition = condition.downcast_ref::<PtyBool>().expect("Expected Bool");
            condition.0
        } {
            self.execute_nodes(block);
        }
    }
    pub fn for_loop(&self, target: &Arc<str>, iter: &Node, block: &[Node]) {
        let iter = self.evaluate(iter);
        let iter = iter.call_method(self, "__iter__", FuncArgs(&[&iter]));

        loop {
            let next = iter.call_method(self, "__next__", FuncArgs(&[&iter]));
            let Some(option) = next.downcast::<PtyOption>() else {
                todo!("{next}");
            };

            if let Some(value) = &option.0 {
                self.get().fields.write(target.clone(), value.clone());
                for node in block {
                    self.evaluate(node);
                }
            } else {
                break;
            };
        }
    }
    pub fn class_def(&self, name: Arc<str>, fields: Arc<[Arc<str>]>, methods: &Arc<[Node]>) {
        let class = PettyClass::new(fields, methods.clone());
        self.get().fields.write(name, class.into());
    }
    pub fn create_literal(&self, literal: &Literal) -> PettyObject {
        match literal {
            #[allow(clippy::cast_sign_loss)]
            Literal::Int(int @ 0..=255) => self.get().preallocated.get(*int as usize).unwrap(),
            #[allow(clippy::cast_precision_loss)]
            Literal::Int(int) => PtyNum(*int as f64).into(),
            Literal::Float(float) => PtyNum(*float).into(),
            Literal::Null => NULL.clone(),
            Literal::Bool(bool) => PtyBool::new(*bool),
            Literal::String(string) => PtyStr(string.clone()).into(),
            Literal::List(list) => PtyList(Mutex::new(self.evaluate_list(list)).into()).into(),
        }
    }
}

impl BinOp {
    #[must_use]
    #[inline]
    /// # Panics
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
    /// # Panics
    pub fn into_petty_function(self) -> &'static str {
        match self {
            Self::Neg => "__neg__",
            Self::Not => "__not__",
            Self::Plus => panic!("Idk what to use this for yet"),
        }
    }
}

impl Deref for Vm {
    type Target = Arc<Mutex<VirtualMachine>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
