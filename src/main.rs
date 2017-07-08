extern crate esprit;
extern crate easter;
extern crate rustyline;
extern crate ansi_term;

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
    match esprit::script("") {
        Err(why) => panic!("Could not compile {:?}", why),
        Ok(ast) => println!("{:#?}", ast.body),
    };
}

#[cfg(not(test))]
fn run_repl() {
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                if !line.is_empty() {
                    compile_repl(&line);
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
fn compile_repl(code: &str) {
    let image: bytecode::Image;

    match esprit::script(code) {
        Err(why) => panic!("Could not compile {:?}", why),
        Ok(ast) => image = bytecode::compile_to_image(ast.body)
    };

    // println!("{:#?}", image);

    let mut engine = vm::VM::new(image);
    engine.run();

    let retval = engine.read_stack_end();
    println!("{}", RGB(130, 130, 130).paint(vm::repl::ret_value_fmt(&retval)));
}
