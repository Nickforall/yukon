mod operations;
mod temp;
pub mod types;
pub mod repl;
pub mod scope;

use self::scope::Scope;
use super::bytecode;
use super::bytecode::Instruction;

#[derive(Debug, PartialEq, Clone)]
pub enum JsValue {
    JsNull,
    JsUndefined,
    JsNan,
    JsNumber(f64),
    JsString(String),
    JsTrue,
    JsFalse,
}

pub struct VM<'a> {
    pub image: bytecode::Image,
    pub stack: Vec<JsValue>,
    pub scope: &'a mut Scope,
    pub sp: usize,
    pub cp: usize
}

impl<'a> VM<'a> {
    pub fn new(img: bytecode::Image, scope: &'a mut Scope) -> VM<'a> {
        VM::<'a> { image: img, stack: Vec::new(), scope: scope, sp: 0, cp: 0 }
    }

    pub fn read_stack_end(&mut self) -> JsValue {
        if self.stack.len() < 1 {
            return JsValue::JsUndefined
        }

        return self.stack[self.stack.len() - 1].clone()
    }

    pub fn push_stack(&mut self, val: JsValue) {
        self.stack.push(val);
    }

    pub fn pop_stack(&mut self) -> JsValue {
        self.stack.pop().unwrap()
    }

    pub fn run (&mut self) {
        let script = self.image.script.instructions.clone();

        for instruction in script {
            match instruction {
                Instruction::PUSHNUM(num) => {
                    self.push_stack(JsValue::JsNumber(num))
                },
                Instruction::ADD => {
                    let a: JsValue = self.pop_stack();
                    let b: JsValue = self.pop_stack();
                    self.push_stack(operations::add(&a, &b))
                },
                Instruction::SUB => {
                    let a: JsValue = self.pop_stack();
                    let b: JsValue = self.pop_stack();
                    self.push_stack(operations::sub(&a, &b))
                },
                Instruction::MLP => {
                    let a: JsValue = self.pop_stack();
                    let b: JsValue = self.pop_stack();
                    self.push_stack(operations::mlp(&a, &b))
                },
                Instruction::DIV => {
                    let a: JsValue = self.pop_stack();
                    let b: JsValue = self.pop_stack();
                    self.push_stack(operations::div(&a, &b))
                },
                Instruction::EQ => {
                    let a: JsValue = self.pop_stack();
                    let b: JsValue = self.pop_stack();
                    self.push_stack(operations::eq(&a, &b))
                },
                Instruction::NEQ => {
                    let a: JsValue = self.pop_stack();
                    let b: JsValue = self.pop_stack();
                    self.push_stack(operations::neq(&a, &b))
                },
                Instruction::SEQ => {
                    let a: JsValue = self.pop_stack();
                    let b: JsValue = self.pop_stack();
                    self.push_stack(operations::strict_eq(&a, &b))
                },
                Instruction::SNEQ => {
                    let a: JsValue = self.pop_stack();
                    let b: JsValue = self.pop_stack();
                    self.push_stack(operations::strict_neq(&a, &b))
                },
                Instruction::PUSHSTRLIT(ref string) => {
                    self.push_stack(JsValue::JsString(string.clone()))
                },
                Instruction::PUSHTRUE => {
                    self.push_stack(JsValue::JsTrue)
                }
                Instruction::PUSHFALSE => {
                    self.push_stack(JsValue::JsFalse)
                },
                Instruction::PUSHVAR(ref string) => {
                    let a = self.pop_stack();
                    self.scope.set_var(string.clone(), a);
                },
                Instruction::READIDENT(ref string) => {
                    let a = self.scope.get_var(string.clone());
                    self.push_stack(a);
                },
                Instruction::ASSIGNEQ(ref string) => {
                    let a = self.pop_stack();
                    self.scope.set_var(string.clone(), a);
                },
                Instruction::ASSIGNPLUSEQ(ref string) => {
                    let a = self.scope.get_var(string.clone());
                    let b = self.pop_stack();
                    self.scope.set_var(string.clone(), operations::add(&a, &b));
                    self.push_stack(operations::add(&a, &b));
                },
                Instruction::ASSIGNSUBEQ(ref string) => {
                    let a = self.scope.get_var(string.clone());
                    let b = self.pop_stack();
                    self.scope.set_var(string.clone(), operations::sub(&a, &b));
                    self.push_stack(operations::sub(&a, &b));
                },
                Instruction::ASSIGNMLPEQ(ref string) => {
                    let a = self.scope.get_var(string.clone());
                    let b = self.pop_stack();
                    self.scope.set_var(string.clone(), operations::mlp(&a, &b));
                    self.push_stack(operations::mlp(&a, &b));
                },
                Instruction::ASSIGNDIVEQ(ref string) => {
                    let a = self.scope.get_var(string.clone());
                    let b = self.pop_stack();
                    self.scope.set_var(string.clone(), operations::div(&a, &b));
                    self.push_stack(operations::div(&a, &b));
                },
                _ => panic!("Unknown instruction in bytecode"),
            };

            // println!("{:?} => {:?}", self.sp, self.stack);
        }
    }
}
