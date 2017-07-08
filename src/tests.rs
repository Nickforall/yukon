#[cfg(test)]
mod bytecode_tests {
    use bytecode::*;
    use esprit;

    pub fn compile_or_panic(code: &str) -> Image {
        let image: Image;

        match esprit::script(code) {
            Err(why) => panic!("Could not compile, {:?}", why),
            Ok(ast) => image = compile_to_image(ast.body)
        };

        return image;
    }

    #[test]
    fn bytecode_binaryop_plus() {
        let image: Image;

        assert_eq!(Image {
            script: Block {instructions: vec![Instruction::PUSHNUM(1 as f64), Instruction::PUSHNUM(10 as f64), Instruction::ADD]},
            blocks: vec![],
        }, compile_or_panic("10 + 1"));
    }

    #[test]
    fn bytecode_binaryop_minus() {
        let image: Image;

        assert_eq!(Image {
            script: Block {instructions: vec![Instruction::PUSHNUM(1 as f64), Instruction::PUSHNUM(10 as f64), Instruction::SUB]},
            blocks: vec![],
        }, compile_or_panic("10 - 1"));
    }

    #[test]
    fn bytecode_binaryop_divide() {
        let image: Image;

        assert_eq!(Image {
            script: Block {instructions: vec![Instruction::PUSHNUM(1 as f64), Instruction::PUSHNUM(10 as f64), Instruction::DIV]},
            blocks: vec![],
        }, compile_or_panic("10 / 1"));
    }

    #[test]
    fn bytecode_binaryop_multiply() {
        let image: Image;

        assert_eq!(Image {
            script: Block {instructions: vec![Instruction::PUSHNUM(1 as f64), Instruction::PUSHNUM(10 as f64), Instruction::MLP]},
            blocks: vec![],
        }, compile_or_panic("10 * 1"));
    }

    #[test]
    fn bytecode_helloworld_string() {
        let image: Image;

        assert_eq!(Image {
            script: Block {instructions: vec![Instruction::PUSHSTRLIT("hello, world".to_owned())]},
            blocks: vec![],
        }, compile_or_panic("\"hello, world\""));
    }
}

#[cfg(test)]
mod vm_tests {
    use bytecode::*;
    use esprit;
    use vm;

    fn compile_repl(code: &str) -> vm::JS_value {
        let image: Image;

        match esprit::script(code) {
            Err(why) => panic!("Could not compile {:?}", why),
            Ok(ast) => image = compile_to_image(ast.body)
        };

        // println!("{:#?}", image);

        let mut engine = vm::VM::new(image);
        engine.run();

        return engine.read_stack_end();
    }

    #[test]
    fn vm_add_numnum() {
        assert_eq!(compile_repl("1 + 2"), vm::JS_value::JS_NUMBER(3 as f64))
    }

    #[test]
    fn vm_mlp_valid() {
        assert_eq!(compile_repl("1 * 2"), vm::JS_value::JS_NUMBER(2 as f64))
    }

    #[test]
    fn vm_div_valid() {
        assert_eq!(compile_repl("10 / 2"), vm::JS_value::JS_NUMBER(5 as f64))
    }

    #[test]
    fn vm_sub_valid() {
        assert_eq!(compile_repl("10 - 2"), vm::JS_value::JS_NUMBER(8 as f64))
    }

    #[test]
    fn vm_concat_strnum() {
        assert_eq!(compile_repl("10 + 'hello'"), vm::JS_value::JS_STRING("10hello".to_owned()))
    }

    #[test]
    fn vm_concat_strstr() {
        assert_eq!(compile_repl("'hello, ' + 'world'"), vm::JS_value::JS_STRING("hello, world".to_owned()))
    }

    #[test]
    fn vm_sub_invalid() {
        assert_eq!(compile_repl("10 - \"\""), vm::JS_value::JS_NAN)
    }

    #[test]
    fn vm_mlp_invalid() {
        assert_eq!(compile_repl("10 * \"\""), vm::JS_value::JS_NAN)
    }

    #[test]
    fn vm_div_invalid() {
        assert_eq!(compile_repl("10 / \"\""), vm::JS_value::JS_NAN)
    }

}
