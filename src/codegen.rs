// BF -> x86-64 System V assembly
use crate::instructions::Instruction;

pub struct Codegen {
  code: Vec<Instruction>,
  code_index: usize,
  label_index: usize,
  pub output: String
}

impl Codegen {
  fn emit_for_current_instruction (&mut self) {
    let instr = &self.code[self.code_index];

    match instr {
      Instruction::Left => self.emit("\nadd $8, %r8"),
      Instruction::Right => self.emit_for_right(),
      Instruction::Increment => self.emit_for_increment(),
      Instruction::Decrement => self.emit_for_decrement(),
      Instruction::Output => self.emit_for_output(),
      Instruction::Input => self.emit_for_input(),
      _ => unimplemented!()
    }
  }

  fn emit_for_input (&mut self) {

  }

  fn emit_for_right (&mut self) {
    // This right operator automagically expands the tape when you
    // go further right than you ever have before.
    let lbl = self.get_unique_label();
    self.emit("
sub $8, %r8
cmp %rsp, %r8");
    self.emit(&format!("
jnb {}", lbl)[..]);
    self.emit("
push $0");
    self.emit(&format!("
{}:", lbl)[..])
  }

  fn emit_for_output (&mut self) {
    self.emit("
mov (%r8), %rdi
push %r8
call _putchar
pop %r8")
  }

  fn emit_for_increment (&mut self) {
    self.emit("
mov (%r8), %rax
inc %rax
mov %rax, (%r8)")
  }

  fn emit_for_decrement (&mut self) {
    self.emit("
mov (%r8), %rax
dec %rax
mov %rax, (%r8)")
  }

  fn emit_program_prelude (&mut self) {
    self.emit("
.globl _main
_main:
push %rbp
mov %rsp, %rbp
mov %rbp, %r8
movq $0, (%r8)")
  }

  fn emit_program_epilogue (&mut self) {
    self.emit("
mov (%r8), %rax
mov %rbp, %rsp
pop %rbp
ret")
  }

  fn emit (&mut self, asm_string: &str) {
    self.output = self.output.clone() + asm_string
  }
  
  fn get_unique_label(&mut self) -> String {
    self.label_index += 1;
    format!("_lbl_{}", self.label_index)
  }

  pub fn generate(&mut self) {
    self.emit_program_prelude();
    while self.code_index < self.code.len() {
      self.emit_for_current_instruction();
      self.code_index += 1;
    }
    self.emit_program_epilogue();
  }

  pub fn new (code: Vec<Instruction>) -> Codegen {
    Codegen { 
      code, 
      code_index: 0,
      label_index: 0,
      output: String::new() 
    }
  }
}