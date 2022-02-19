// BF -> x86-64 System V assembly
use crate::instructions::Instruction;

pub struct Codegen {
  code: Vec<Instruction>,
  code_index: usize,
  label_index: usize,
  loop_index: usize,
  open_loops: Vec<usize>,
  pub output: String
}

impl Codegen {
  fn emit_for_current_instruction (&mut self) {
    let instr = &self.code[self.code_index - 1];

    // TODO: We can look for common operations such as [-], and
    //   turn them into "set 0"
    match instr {
      Instruction::Left => self.emit_for_left(),
      Instruction::Right => self.emit_for_right(),
      Instruction::Increment => self.emit_for_increment(),
      Instruction::Decrement => self.emit_for_decrement(),
      Instruction::Output => self.emit_for_output(),
      Instruction::Input => self.emit_for_input(),
      Instruction::LoopStart => self.emit_for_loop_start(),
      Instruction::LoopEnd => self.emit_for_loop_end(),
    }
  }

  fn swallow_and_count_repeating (&mut self, instr: Instruction) -> usize {
    let mut count = 0;
    while self.code_index < self.code.len() {
      if self.code[self.code_index] == instr {
        count += 1;
        self.code_index += 1;
      } else { break }
    }
    count
  }

  fn emit_for_left (&mut self) {
    let repeated_lefts = 1 + self.swallow_and_count_repeating(Instruction::Left);
    self.emit(&format!("\nadd ${}, %r12", 8 * repeated_lefts)[..])
  }

  fn emit_for_loop_start (&mut self) {
    self.emit("
cmpq $0, (%r12)");
    self.emit(&format!("\nje _lbl_end_{}", self.loop_index)[..]);
    self.emit(&format!("\n_lbl_start_{}:", self.loop_index)[..]);
    self.open_loops.push(self.loop_index);
    self.loop_index += 1;
  }

  fn emit_for_loop_end (&mut self) {
    let to_close = self.open_loops.pop().unwrap();
    self.emit("
cmpq $0, (%r12)");
    self.emit(&format!("\njne _lbl_start_{}", to_close)[..]);
    self.emit(&format!("\n_lbl_end_{}:", to_close)[..]);
  }

  fn emit_for_input (&mut self) {
    self.emit("
call _getchar
mov %rax, (%r12)")
  }

  fn emit_for_right (&mut self) {
    // This right operator automagically expands the tape when you
    // go further right than you ever have before.
    // TODO: Due to this memory management solution, we are not currently
    //   able to batch right operations. We _could_ with some clever asm
    //   maths to calculate the difference between %rsp and %r12 and multiplying
    //   that by 8, but we will not implement that for now.
    let lbl = self.get_unique_label();
    self.emit("
sub $8, %r12
cmp %rsp, %r12");
    self.emit(&format!("
jnb {}", lbl)[..]);
    self.emit("
push $0");
    self.emit(&format!("
{}:", lbl)[..])
  }

  fn emit_for_output (&mut self) {
    self.emit("
mov (%r12), %rdi
call _putchar")
  }

  fn emit_for_increment (&mut self) {
    let repeated_incs = 1 + self.swallow_and_count_repeating(Instruction::Increment);
    self.emit("
mov (%r12), %rax");
    self.emit(&format!("\nadd ${}, %rax", repeated_incs)[..]);
    self.emit("\nmov %rax, (%r12)")
  }

  fn emit_for_decrement (&mut self) {
    let repeated_decs = 1 + self.swallow_and_count_repeating(Instruction::Decrement);
    self.emit("
mov (%r12), %rax");
    self.emit(&format!("\nsub ${}, %rax", repeated_decs)[..]);
    self.emit("\nmov %rax, (%r12)")
  }

  fn emit_program_prelude (&mut self) {
    self.emit("
.globl _main
_main:
push %rbp
mov %rsp, %rbp
mov %rbp, %r12
movq $0, (%r12)")
  }

  fn emit_program_epilogue (&mut self) {
    self.emit("
mov (%r12), %rax
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
      self.code_index += 1;
      self.emit_for_current_instruction();
    }
    self.emit_program_epilogue();
  }

  pub fn new (code: Vec<Instruction>) -> Codegen {
    Codegen { 
      code, 
      code_index: 0,
      label_index: 0,
      loop_index: 0,
      open_loops: vec![],
      output: String::new() 
    }
  }
}
