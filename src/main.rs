mod codegen;
mod instructions;

use std::env;
use codegen::Codegen;

fn main() {
    let filename = env::args().nth(1)
        .expect("Pass a .bf file path argument");
    let instrs = instructions::read_from_file(&filename);

    if let Ok(i) = instrs {
        let mut gen = Codegen::new(i);
        gen.generate();
        print!("{}", gen.output)
    } else {
        println!("Failed to lex {:?}", instrs.err().unwrap())
    }
}
