extern crate esprit;
extern crate easter;
extern crate rustyline;
extern crate ansi_term;
extern crate joker;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use ansi_term::Colour::RGB;

use std::env;

mod bytecode;
mod tests;
mod vm;

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "repl" {
        run_repl();
    } else {
        devel();
    }
}

#[cfg(not(test))]
fn devel() {
    match esprit::script("a++") {
        Err(why) => panic!("Could not compile {:?}", why),
        Ok(ast) => println!("{:#?}", ast.body),
    };
}

#[cfg(not(test))]
fn run_repl() {
    let mut rl = Editor::<()>::new();
    let mut scope = vm::scope::Scope::new_global();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if !line.is_empty() {
                    compile_repl(&line, &mut scope);
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}

#[cfg(not(test))]
fn compile_repl(code: &str, mut scope: &mut vm::scope::Scope) {
    let image: bytecode::Image;

    match esprit::script(code) {
        Err(why) => panic!("Could not compile {:?}", why),
        Ok(ast) => image = bytecode::compile_to_image(ast.body)
    };
    let mut engine = vm::VM::new(image, &mut scope);
    engine.run();

    let retval = engine.read_stack_end();
    println!("{}", RGB(130, 130, 130).paint(vm::repl::ret_value_fmt(&retval)));
}
