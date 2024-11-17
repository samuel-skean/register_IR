use instructions::Instruction;
use registers::RegisterFile;

pub mod registers;
pub mod instructions;

pub fn run(program: &[Instruction]) {
    let mut rf = RegisterFile::new();
    run_against_rf(&mut rf, program);    
}

fn run_against_rf(rf: &mut RegisterFile, program: &[Instruction]) {
    let label_indices = program
        .iter()
        .enumerate()
        .filter_map(|(i, instr)| match instr {
            &Instruction::Label(l) => Some((l, i)),
            _ => None,
        }).collect();
    while rf.program_counter < program.len() {
        program[rf.program_counter].run(rf, &label_indices);
    }
}