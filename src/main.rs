extern crate rustyline;

mod vm;
mod parser;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use vm::VirtualMachine;

fn main() {
    println!(" ____               _");
    println!("|  _ \\             | |");
    println!("| |_) | _   _  ___ | |_");
    println!("|  _ < | | | |/ __|| __|");
    println!("| |_) || |_| |\\__ \\| |_");
    println!("|____/  \\__,_||___/ \\__|\n");
    println!("Brainfuck REPL");
    println!("Written by Mateusz 'Haggus' Mrowiec\n");
    println!("Press Ctrl+C to Exit\n");

    let mut vm = VirtualMachine::new();
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                vm.add_instructions(line.as_str());
                println!("{}", vm.run().unwrap());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
