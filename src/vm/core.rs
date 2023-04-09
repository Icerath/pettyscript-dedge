use super::{
    petty_class::PettyClass, petty_function::PettyFunction, preallocated::PreAllocated, prelude::*,
};
use crate::ast::{BinOp, Literal, Node, UnaryOp};
use std::ops::Deref;

#[derive(Default, Clone)]
pub struct VirtualMachine {
    pub preallocated: PreAllocated,
    pub globals: Arc<Mutex<Dict>>,
}

#[derive(Default)]
pub struct Vm {
    pub inner: VirtualMachine,
    pub return_val: Option<PettyObject>,
    pub scopes: Vec<Dict>,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_builtin(&mut self, name: &str, object: PettyObject) {
        self.write(name.into(), object);
    }

    pub fn write(&mut self, key: Arc<str>, value: PettyObject) {
        match self.scopes.last_mut() {
            Some(scope) => scope.insert(key, value),
            None => self.globals.lock().unwrap().insert(key, value),
        };
    }

    pub fn read(&mut self, key: &str) -> PettyObject {
        for scope in self.scopes.iter().rev() {
            if let Some(object) = scope.get(key) {
                return object.clone();
            }
        }
        self.globals
            .lock()
            .unwrap()
            .get(key)
            .unwrap_or_else(|| panic!("Not found: ({key})"))
            .clone()
    }

    pub fn new_scope(&mut self) {
        self.scopes.push(Dict::new());
    }

    pub fn drop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn spawn_thread(&self) -> Self {
        Vm {
            inner: self.inner.clone(),
            return_val: None,
            scopes: vec![],
        }
    }
}

impl Vm {
    pub fn evaluate(&mut self, node: &Node) -> PettyObject {
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
            Node::Ident(ident) => return self.read(ident),
            Node::FuncCall(name, args) => return self.func_call(name, args),
            Node::FuncDef(name, args, block) => {
                self.func_def(name.clone(), args.clone(), block.clone());
            }
            Node::ReturnState(expr) => self.return_val = Some(self.evaluate(expr)),
            Node::UnaryOp(op, expr) => return self.unary_expr(*op, expr),
            Node::IfState(condition, block, or_else) => {
                self.if_statement(condition, block, or_else.as_ref().map(Arc::as_ref));
            }
            Node::WhileLoop(condition, block) => self.while_loop(condition, block),
            Node::ForLoop(target, iter, block) => self.for_loop(target, iter, block),
            Node::ClassDef(name, fields, methods) => {
                self.class_def(name.clone(), fields.clone(), methods.clone());
            }
            Node::Closure(params, body) => return self.closure(params.clone(), body.clone()),
            Node::GetItemIndex(ident, expr) => return self.get_item_index(ident, expr),
            Node::SetItemIndex(ident, index, expr) => self.set_item_index(ident, index, expr),
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

    pub fn set_eq(&mut self, name: Arc<str>, expr: &Node) {
        let value = self.evaluate(expr);
        self.write(name, value);
    }

    pub fn get_item(&mut self, left: &Node, right: &Node) -> PettyObject {
        let left = self.evaluate(left);

        let (function, args) = match right {
            Node::Ident(ident) => return left.get_item(self, &left, ident),
            Node::FuncCall(name, args) => (left.get_item(self, &left, name), args),
            _ => unreachable!(),
        };

        let mut items = Vec::with_capacity(args.len());
        if left.downcast_ref::<Module>().is_none() {
            items.push(left);
        }
        items.extend(args.iter().map(|node| self.evaluate(node)));

        let refs: Vec<_> = items.iter().collect();
        function.call(self, &function, FuncArgs(&refs))
    }

    pub fn bin_expr(&mut self, op: BinOp, lhs: &Node, rhs: &Node) -> PettyObject {
        let lhs = self.evaluate(lhs);
        let rhs = self.evaluate(rhs);
        let function_name = op.into_petty_function();
        let function = lhs.get_item(self, &lhs, function_name);
        let binding = [&lhs, &rhs];
        let args = FuncArgs(&binding);
        function.call(self, &function, args)
    }

    pub fn unary_expr(&mut self, op: UnaryOp, expr: &Node) -> PettyObject {
        let inner = self.evaluate(expr);
        let function_name = op.into_petty_function();
        let function = inner.get_item(self, &inner, function_name);
        let binding = [&inner];
        let args = FuncArgs(&binding);
        function.call(self, &function, args)
    }

    pub fn func_call(&mut self, name: &str, args: &[Node]) -> PettyObject {
        let function = self.read(name);
        let args = self.evaluate_list(args);
        function.call(
            self,
            &function,
            FuncArgs(args.iter().collect::<Vec<_>>().as_slice()),
        )
    }

    pub fn evaluate_list(&mut self, items: &[Node]) -> Vec<PettyObject> {
        items.iter().map(|arg| self.evaluate(arg)).collect()
    }

    pub fn func_def(&mut self, name: Arc<str>, args: Arc<[Arc<str>]>, block: Arc<[Node]>) {
        let function = self.closure(args, block);
        self.write(name, function);
    }

    pub fn closure(&mut self, args: Arc<[Arc<str>]>, block: Arc<[Node]>) -> PettyObject {
        PettyFunction::new(args, block, self.scopes.clone()).into()
    }

    pub fn if_statement(&mut self, condition: &Node, block: &[Node], or_else: Option<&Node>) {
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

    pub fn while_loop(&mut self, condition: &Node, block: &[Node]) {
        while self.return_val.is_none() && {
            let condition = self.evaluate(condition);
            let condition = condition.call_method(self, "__bool__", FuncArgs(&[]));
            let condition = condition.downcast_ref::<PtyBool>().expect("Expected Bool");
            condition.0
        } {
            self.execute_nodes(block);
        }
    }

    pub fn for_loop(&mut self, target: &Arc<str>, iter: &Node, block: &[Node]) {
        let iter = self.evaluate(iter);
        let iter = iter.call_method(self, "__iter__", FuncArgs(&[&iter]));

        let get_next = iter.get_item(self, &iter, "__next__");

        while let Some(next) = {
            let next = get_next.call(self, &get_next, FuncArgs(&[&iter]));
            match next.downcast::<PtyOption>() {
                Some(next) => next.0,
                None => todo!("{next}"),
            }
        } {
            self.write(target.clone(), next.clone());
            for node in block {
                self.evaluate(node);
            }
        }
    }

    pub fn class_def(&mut self, name: Arc<str>, fields: Arc<[Arc<str>]>, methods: Arc<[Node]>) {
        let class = PettyClass::new(fields, methods);
        self.write(name, class.into());
    }

    pub fn get_item_index(&mut self, ident: &Arc<str>, expr: &Node) -> PettyObject {
        let value = self.evaluate(expr);
        let object = self.read(ident);
        object.call_method(self, "__get_index__", FuncArgs(&[&object, &value]))
    }
    pub fn set_item_index(&mut self, ident: &Arc<str>, index: &Node, expr: &Node) {
        let index = self.evaluate(index);
        let value = self.evaluate(expr);
        let object = self.read(ident);
        object.call_method(self, "__set_index__", FuncArgs(&[&object, &index, &value]));
    }

    pub fn create_literal(&mut self, literal: &Literal) -> PettyObject {
        match literal {
            #[allow(clippy::cast_sign_loss)]
            Literal::Int(int @ 0..=255) => self.preallocated.get(*int as usize).unwrap(),
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
    type Target = VirtualMachine;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
