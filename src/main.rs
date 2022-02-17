mod codegen;
mod instructions;

use codegen::Codegen;

fn main() {
    let instrs = instructions::read_from_file("test.bf");

    if let Ok(i) = instrs {
        let mut gen = Codegen::new(i);
        gen.generate();
        print!("{}", gen.output)
    } else {
        println!("Failed to lex 'test.bf' {:?}", instrs.err().unwrap())
    }
}
