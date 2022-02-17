use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub enum Instruction {
  Left,
  Right,
  Increment,
  Decrement,
  Output,
  Input,
  LoopStart,
  LoopEnd
}

// This is basically a 'lexer'/'parser' for BF
pub fn read_from_file (pth: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
  let mut f = File::open(pth)?;
  let mut buffer = String::new();
  f.read_to_string(&mut buffer)?;

  // buffer is now a string from that file
  let mut instrs: Vec<Instruction> = Vec::new();
  for c in buffer.chars() {
    match c {
      '+' => instrs.push(Instruction::Increment),
      '-' => instrs.push(Instruction::Decrement),
      '<' => instrs.push(Instruction::Left),
      '>' => instrs.push(Instruction::Right),
      '.' => instrs.push(Instruction::Output),
      ',' => instrs.push(Instruction::Input),
      '[' => instrs.push(Instruction::LoopStart),
      ']' => instrs.push(Instruction::LoopEnd),
      // Anything else is a comment, ignore it
      _ => {},
    }
  }

  Ok(instrs)
}
