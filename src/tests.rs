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

    pub fn compile_repl(code: &str) -> vm::JsValue {
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

    mod binary_operations {
        use super::compile_repl;
        use super::vm;

        #[test]
        fn vm_add_numnum() {
            assert_eq!(compile_repl("1 + 2"), vm::JsValue::JsNumber(3 as f64))
        }

        #[test]
        fn vm_mlp_valid() {
            assert_eq!(compile_repl("1 * 2"), vm::JsValue::JsNumber(2 as f64))
        }

        #[test]
        fn vm_div_valid() {
            assert_eq!(compile_repl("10 / 2"), vm::JsValue::JsNumber(5 as f64))
        }

        #[test]
        fn vm_sub_valid() {
            assert_eq!(compile_repl("10 - 2"), vm::JsValue::JsNumber(8 as f64))
        }

        #[test]
        fn vm_concat_strnum() {
            assert_eq!(compile_repl("10 + 'hello'"), vm::JsValue::JsString("10hello".to_owned()))
        }

        #[test]
        fn vm_concat_strstr() {
            assert_eq!(compile_repl("'hello, ' + 'world'"), vm::JsValue::JsString("hello, world".to_owned()))
        }

        #[test]
        fn vm_sub_invalid() {
            assert_eq!(compile_repl("10 - \"\""), vm::JsValue::JsNan)
        }

        #[test]
        fn vm_mlp_invalid() {
            assert_eq!(compile_repl("10 * \"\""), vm::JsValue::JsNan)
        }

        #[test]
        fn vm_div_invalid() {
            assert_eq!(compile_repl("10 / \"\""), vm::JsValue::JsNan)
        }

        #[test]
        fn vm_sub_numstr() {
            assert_eq!(compile_repl("10 - \"5\""), vm::JsValue::JsNumber(5 as f64))
        }

        #[test]
        fn vm_mlp_numstr() {
            assert_eq!(compile_repl("10 * \"5\""), vm::JsValue::JsNumber(50 as f64))
        }

        #[test]
        fn vm_div_numstr() {
            assert_eq!(compile_repl("10 / \"5\""), vm::JsValue::JsNumber(2 as f64))
        }

        #[test]
        fn vm_sub_numstr_invalid() {
            assert_eq!(compile_repl("10 - \"a\""), vm::JsValue::JsNan)
        }

        #[test]
        fn vm_mlp_numstr_invalid() {
            assert_eq!(compile_repl("10 * \"a\""), vm::JsValue::JsNan)
        }

        #[test]
        fn vm_div_numstr_invalid() {
            assert_eq!(compile_repl("10 / \"a\""), vm::JsValue::JsNan)
        }

        #[test]
        fn vm_sub_strnum() {
            assert_eq!(compile_repl("\"10\" - 5"), vm::JsValue::JsNumber(5 as f64))
        }

        #[test]
        fn vm_mlp_strnum() {
            assert_eq!(compile_repl("\"5\" * 10"), vm::JsValue::JsNumber(50 as f64))
        }

        #[test]
        fn vm_div_strnum() {
            assert_eq!(compile_repl("\"10\" / 5"), vm::JsValue::JsNumber(2 as f64))
        }

        #[test]
        fn vm_sub_strnum_invalid() {
            assert_eq!(compile_repl("\"a\" - 5"), vm::JsValue::JsNan)
        }

        #[test]
        fn vm_mlp_strnum_invalid() {
            assert_eq!(compile_repl("\"a\" * 10"), vm::JsValue::JsNan)
        }

        #[test]
        fn vm_div_strnum_invalid() {
            assert_eq!(compile_repl("\"a\" / 5"), vm::JsValue::JsNan)
        }
    }

    mod booleans {
        use super::compile_repl;
        use super::vm;

        #[test]
        fn num_eq_strnum() {
            assert_eq!(compile_repl("1 == '1'"), vm::JsValue::JsTrue)
        }

        #[test]
        fn num_neq_strnum() {
            assert_eq!(compile_repl("1 != '2'"), vm::JsValue::JsTrue)
        }

        #[test]
        fn num_eq_numstr() {
            assert_eq!(compile_repl("'1' == 1"), vm::JsValue::JsTrue)
        }

        #[test]
        fn num_neq_numstr() {
            assert_eq!(compile_repl("'2' != 1"), vm::JsValue::JsTrue)
        }

        #[test]
        fn num_seq_strnum() {
            assert_eq!(compile_repl("1 === '1'"), vm::JsValue::JsFalse)
        }

        #[test]
        fn num_sneq_strnum() {
            assert_eq!(compile_repl("1 !== '1'"), vm::JsValue::JsTrue)
        }

        #[test]
        fn num_seq_numstr() {
            assert_eq!(compile_repl("'1' === 1"), vm::JsValue::JsFalse)
        }

        #[test]
        fn num_sneq_numstr() {
            assert_eq!(compile_repl("'1' !== 1"), vm::JsValue::JsTrue)
        }

        #[test]
        fn bool_empty_string() {
            assert_eq!(compile_repl("true == ''"), vm::JsValue::JsFalse)
        }

        #[test]
        fn empty_string_bool() {
            assert_eq!(compile_repl("'' == false"), vm::JsValue::JsTrue)
        }

        #[test]
        fn num_sneq() {
            assert_eq!(compile_repl("2 !== 1"), vm::JsValue::JsTrue)
        }

        #[test]
        fn num_seq() {
            assert_eq!(compile_repl("1 === 1"), vm::JsValue::JsTrue)
        }

        #[test]
        fn str_seq() {
            assert_eq!(compile_repl("'HELLO' === 'HELLO'"), vm::JsValue::JsTrue)
        }

        #[test]
        fn str_eq() {
            assert_eq!(compile_repl("'HELLO' == 'HELLO'"), vm::JsValue::JsTrue)
        }

        #[test]
        fn zerostr_true() {
            assert_eq!(compile_repl("'0' == false"), vm::JsValue::JsTrue)
        }

        #[test]
        fn zero_true() {
            assert_eq!(compile_repl("0 == false"), vm::JsValue::JsTrue)
        }

        #[test]
        fn seq_true() {
            assert_eq!(compile_repl("true === true"), vm::JsValue::JsTrue)
        }

        #[test]
        fn eq_false() {
            assert_eq!(compile_repl("false == false"), vm::JsValue::JsTrue)
        }

        #[test]
        fn eq_true() {
            assert_eq!(compile_repl("true == true"), vm::JsValue::JsTrue)
        }

        #[test]
        fn seq_false() {
            assert_eq!(compile_repl("false === false"), vm::JsValue::JsTrue)
        }

        #[test]
        fn num_eq() {
            assert_eq!(compile_repl("1 == 1"), vm::JsValue::JsTrue)
        }
    }

}
