use super::{
    builtins::{self, BoolBuiltin, IntBuiltin, NullBuiltin, StringBuiltin},
    value::{PettyValue, PettyValueCustom, PettyValueFunction},
};
use crate::ast::{BinOp, Literal, Node};
use std::collections::HashMap;

#[cfg(debug_assertions)]
pub const RECURSION_LIMIT: usize = 400;
#[cfg(not(debug_assertions))]
pub const RECURSION_LIMIT: usize = 1000;
pub struct Interpreter {
    pub variables: ProgramVariables,
    pub null: PettyValue,
    pub return_val: Option<PettyValue>,
}
pub struct ProgramVariables {
    scoped: Vec<HashMap<Box<str>, PettyValue>>,
    globals: HashMap<Box<str>, PettyValue>,
}

impl Interpreter {
    pub fn init() -> Self {
        Self {
            variables: ProgramVariables::init(),
            null: NullBuiltin.into(),
            return_val: None,
        }
    }
    pub fn load_builtin(&mut self, name: Box<str>, builtin: PettyValue) {
        let globals = &mut self.variables.globals;
        globals.insert(name, builtin);
    }
    pub fn evaluate(&mut self, node: &Node) -> PettyValue {
        match node {
            Node::Globals(nodes) | Node::Block(nodes) => self.execute_nodes(nodes),
            Node::BinExpr(op, nodes) => {
                let left = self.evaluate(&nodes.0);
                let right = self.evaluate(&nodes.1);
                return self.evaluate_bin_expr(*op, left, right);
            }
            Node::BreakState => todo!(),
            Node::ClassDef(name, fields, node_methods) => {
                let mut methods = HashMap::new();
                for node in node_methods.iter() {
                    let Node::FuncDef(method_name, params, block) = node else {
                        unreachable!()
                    };
                    let func = PettyValueFunction::new(params.to_vec(), block.clone()).into();
                    methods.insert(method_name.clone(), func);
                }
                self.create_class(name.clone(), fields.clone(), methods);
            }
            Node::Empty => todo!(),
            Node::ForLoop(target, iter, block) => {
                let iter = self.evaluate(iter);
                self.for_loop(target.clone(), iter, block);
            }
            Node::FuncCall(name, args) => {
                let args = args
                    .iter()
                    .map(|node| self.evaluate(node))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                return self.call_function(name, args);
            }
            Node::FuncDef(name, params, block) => {
                self.create_function(name.clone(), params.clone(), block.clone());
            }
            Node::Ident(ident) => return self.read_ident(ident),
            Node::IfState(condition, block, or_else) => {
                self.if_statement(condition, block, or_else.as_ref().map(Box::as_ref));
            }
            Node::Literal(literal) => return create_literal(literal),
            Node::ReturnState(expr) => self.return_statement(expr),
            Node::SetEq(var, expr) => {
                let value = self.evaluate(expr);
                self.set_eq(var.clone(), value);
            }
            Node::UnaryOp(op, expr) => todo!(),
            Node::WhileLoop(condition, body) => self.while_loop(condition, body),
        };
        self.null.clone()
    }
    pub fn evaluate_bin_expr(
        &mut self,
        op: BinOp,
        left: PettyValue,
        right: PettyValue,
    ) -> PettyValue {
        let output = match op {
            BinOp::Add => left.inner().__add__(self, left.clone(), right.clone()),
            BinOp::Sub => left.inner().__sub__(self, left.clone(), right.clone()),
            BinOp::Mul => left.inner().__mul__(self, left.clone(), right.clone()),
            BinOp::Div => left.inner().__div__(self, left.clone(), right.clone()),
            BinOp::And => left.inner().__and__(self, left.clone(), right.clone()),
            BinOp::GT => left.inner().__gt__(right.clone()),
            BinOp::LT => left.inner().__lt__(right.clone()),
            BinOp::IsEq => left.inner().__is_eq__(right.clone()),
            BinOp::GTEq => left.inner().__gt_eq__(right.clone()),
            BinOp::LTEq => left.inner().__lt_eq__(right.clone()),
            _ => todo!(),
        };
        let panic = || {
            panic!(
                "BinOp: ('{op:?}' | '{}' | '{}')",
                self.repr(left.clone()).unwrap(),
                self.repr(right.clone()).unwrap()
            )
        };
        output.unwrap_or_else(panic)
    }
    pub fn repr(&mut self, value: PettyValue) -> Option<String> {
        let repr = value.inner().__repr__(self, value.clone())?;
        Some(
            repr.inner()
                .as_any()
                .downcast_ref::<StringBuiltin>()?
                .0
                .clone(),
        )
    }
    pub fn set_eq(&mut self, name: Box<str>, value: PettyValue) {
        self.variables.write(name, value);
    }
    pub fn read_ident(&self, name: &str) -> PettyValue {
        self.variables
            .read(name)
            .expect("Variable does not exist. - TODO: Error")
    }
    pub fn call_function(&mut self, name: &str, args: Box<[PettyValue]>) -> PettyValue {
        let function = self.read_ident(name);
        function
            .inner()
            .__call__(self, args.to_vec())
            .unwrap_or_else(|| self.null())
    }
    pub fn create_function(&mut self, name: Box<str>, params: Box<[Box<str>]>, block: Box<[Node]>) {
        let function = PettyValueFunction::new(params.to_vec(), block);
        self.variables.write(name, function.into());
    }
    pub fn create_class(
        &mut self,
        name: Box<str>,
        fields: Box<[Box<str>]>,
        mut methods: HashMap<Box<str>, PettyValue>,
    ) {
        for field in fields.to_vec() {
            methods.insert(field, self.null());
        }

        let class = PettyValueCustom::new(methods);
        self.variables.write(name, class.into());
    }
    pub fn execute_nodes(&mut self, nodes: &[Node]) {
        for node in nodes {
            self.evaluate(node);
        }
    }
    pub fn if_statement(&mut self, condition: &Node, block: &[Node], or_else: Option<&Node>) {
        let condition = self.evaluate(condition);
        let bool_object = condition.clone().inner().__bool__(self, condition).unwrap();
        let boolean = bool_object
            .inner()
            .as_any()
            .downcast_ref::<BoolBuiltin>()
            .unwrap();
        if boolean.0 {
            self.execute_nodes(block);
        } else if let Some(or_else) = or_else {
            self.evaluate(or_else);
        }
    }
    pub fn for_loop(&mut self, target: Box<str>, iter: PettyValue, block: &[Node]) {
        todo!()
    }
    pub fn while_loop(&mut self, condition: &Node, block: &[Node]) {
        while {
            let value = self.evaluate(condition);
            self.petty_bool(&value).unwrap()
        } {
            self.execute_nodes(block);
        }
    }
    pub fn return_statement(&mut self, node: &Node) {
        let value = self.evaluate(node);
        self.return_val = Some(value);
    }

    pub fn null(&self) -> PettyValue {
        self.null.clone()
    }
    pub fn petty_bool(&mut self, value: &PettyValue) -> Option<bool> {
        let inner = value.inner().__bool__(self, value.clone())?;
        Some(inner.inner().as_any().downcast_ref::<BoolBuiltin>()?.0)
    }
}

impl ProgramVariables {
    pub fn init() -> Self {
        let scoped = vec![];
        let globals = HashMap::new();
        Self { scoped, globals }
    }
    pub fn new_scope(&mut self) {
        if self.scoped.len() >= RECURSION_LIMIT {
            todo!("Hit Recursion Limit {RECURSION_LIMIT}");
        }
        self.scoped.push(HashMap::new());
    }
    pub fn drop_scope(&mut self) {
        let scope_exits = self.scoped.pop().is_some();
        assert!(scope_exits);
    }
    pub fn read(&self, str: &str) -> Option<PettyValue> {
        if let Some(output) = self.current_scope().get(str).cloned() {
            return Some(output);
        }
        if let Some(output) = self.globals.get(str).cloned() {
            return Some(output);
        }
        None
    }
    pub fn write(&mut self, str: Box<str>, value: PettyValue) {
        self.current_scope_mut().insert(str, value);
    }
    fn current_scope(&self) -> &HashMap<Box<str>, PettyValue> {
        self.scoped.last().unwrap_or(&self.globals)
    }
    fn current_scope_mut(&mut self) -> &mut HashMap<Box<str>, PettyValue> {
        self.scoped.last_mut().unwrap_or(&mut self.globals)
    }
}

pub fn create_literal(literal: &Literal) -> PettyValue {
    match literal {
        Literal::Int(num) => IntBuiltin(*num).into(),
        Literal::Bool(bool) => BoolBuiltin(*bool).into(),
        Literal::String(string) => StringBuiltin(string.to_string()).into(),
        Literal::Null => NullBuiltin.into(),
        _ => todo!(),
    }
}
