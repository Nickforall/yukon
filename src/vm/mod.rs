mod operations;
mod temp;
pub mod types;
pub mod repl;

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

pub struct VM {
    pub image: bytecode::Image,
    pub stack: Vec<JsValue>,
    pub sp: usize,
    pub cp: usize
}

impl VM {
    pub fn new(img: bytecode::Image) -> VM {
        VM { image: img, stack: Vec::new(), sp: 0, cp: 0 }
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
                }
                _ => panic!("Unknown instruction in bytecode"),
            };

            // println!("{:?} => {:?}", self.sp, self.stack);
        }
    }
}
