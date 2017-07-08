mod operations;
mod temp;
pub mod repl;

use super::bytecode;
use super::bytecode::Instruction;

#[derive(Debug, PartialEq, Clone)]
pub enum JS_value {
    JS_NULL,
    JS_UNDEFINED,
    JS_NAN,
    JS_NUMBER(f64),
    JS_STRING(String),
}

pub struct VM {
    pub image: bytecode::Image,
    pub stack: Vec<JS_value>,
    pub sp: usize,
    pub cp: usize
}

impl VM {
    pub fn new(img: bytecode::Image) -> VM {
        VM { image: img, stack: Vec::new(), sp: 0, cp: 0 }
    }

    pub fn read_stack_end(&mut self) -> JS_value {
        if self.stack.len() < 1 {
            return JS_value::JS_UNDEFINED
        }

        return self.stack[self.stack.len() - 1].clone()
    }

    pub fn push_stack(&mut self, val: JS_value) {
        self.stack.push(val);
    }

    pub fn pop_stack(&mut self) -> JS_value {
        self.stack.pop().unwrap()
    }

    pub fn run (&mut self) {
        let script = self.image.script.instructions.clone();

        for instruction in script {
            match instruction {
                Instruction::PUSHNUM(num) => {
                    self.push_stack(JS_value::JS_NUMBER(num))
                },
                Instruction::ADD => {
                    let a: JS_value = self.pop_stack();
                    let b: JS_value = self.pop_stack();
                    self.push_stack(operations::add(&a, &b))
                },
                Instruction::SUB => {
                    let a: JS_value = self.pop_stack();
                    let b: JS_value = self.pop_stack();
                    self.push_stack(operations::sub(&a, &b))
                },
                Instruction::MLP => {
                    let a: JS_value = self.pop_stack();
                    let b: JS_value = self.pop_stack();
                    self.push_stack(operations::mlp(&a, &b))
                },
                Instruction::DIV => {
                    let a: JS_value = self.pop_stack();
                    let b: JS_value = self.pop_stack();
                    self.push_stack(operations::div(&a, &b))
                },
                Instruction::PUSHSTRLIT(ref string) => {
                    self.push_stack(JS_value::JS_STRING(string.clone()))
                },
                _ => panic!("Unknown instruction in bytecode"),
            };

            // println!("{:?} => {:?}", self.sp, self.stack);
        }
    }
}
