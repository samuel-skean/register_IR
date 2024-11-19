use std::collections::HashMap;

use instructions::Instruction;
use registers::{LabelName, RegisterFile};

pub mod instructions;
pub mod liveness_analysis;
pub mod registers;

pub fn run(program: &[Instruction]) -> RegisterFile {
    let mut rf = RegisterFile::new();
    run_against_rf(&mut rf, program);
    rf
}

fn create_label_indices(program: &[Instruction]) -> HashMap<LabelName, usize> {
    program
        .iter()
        .enumerate()
        .filter_map(|(i, instr)| match instr {
            &Instruction::Label(l) => Some((l, i)),
            _ => None,
        })
        .collect()
}

fn run_against_rf(rf: &mut RegisterFile, program: &[Instruction]) {
    let label_indices = create_label_indices(program);
    while rf.program_counter < program.len() {
        program[rf.program_counter].run(rf, &label_indices);
    }
}

#[cfg(test)]
mod test_consts {
    use registers::{LabelName, RegisterName};

    use super::*;

    pub const R0: RegisterName = RegisterName::with_value(0);
    pub const R1: RegisterName = RegisterName::with_value(1);
    pub const R2: RegisterName = RegisterName::with_value(2);
    pub const R3: RegisterName = RegisterName::with_value(3);

    pub const L0: LabelName = LabelName::with_value(0);
    pub const L1: LabelName = LabelName::with_value(1);

    pub const BASIC_STRAIGHT_LINE: &[Instruction] = &[
        Instruction::LoadImmediate(R1, 90),
        Instruction::Subtract {
            assignee: R1,
            lhs: R1,
            rhs: R0,
        },
        Instruction::LoadImmediate(R2, 100),
        Instruction::Subtract {
            assignee: R1,
            lhs: R2,
            rhs: R1,
        },
    ];

    pub const SIMPLE_BRANCH: &[Instruction] = &[
        Instruction::JumpIfZero(R0, L0),
        Instruction::LoadImmediate(R1, 90), // Doesn't get executed.
        Instruction::Label(L0),
    ];

    const DECREMENT: RegisterName = R1;
    const BEFORE_SIMPLE_LOOP: LabelName = L0;
    const AFTER_SIMPLE_LOOP: LabelName = L1;

    pub const SIMPLE_LOOP: &[Instruction] = &[
        Instruction::LoadImmediate(DECREMENT, 1),
        Instruction::LoadImmediate(R2, 6),
        Instruction::LoadImmediate(R3, 10),
        Instruction::Label(BEFORE_SIMPLE_LOOP),
        Instruction::Subtract {
            assignee: R2,
            lhs: R2,
            rhs: DECREMENT,
        },
        Instruction::Subtract {
            assignee: R3,
            lhs: R3,
            rhs: DECREMENT,
        },
        Instruction::JumpIfZero(R2, AFTER_SIMPLE_LOOP),
        Instruction::JumpIfZero(R0, BEFORE_SIMPLE_LOOP),
        Instruction::Label(AFTER_SIMPLE_LOOP),
    ];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_consts::*;

    #[test]
    fn straight_line() {
        let mut final_rf = run(BASIC_STRAIGHT_LINE);
        assert_eq!(final_rf.get(R0), 0);
        assert_eq!(final_rf.get(R1), 10);
        assert_eq!(final_rf.get(R2), 100);
    }

    #[test]
    fn simple_branch() {
        let mut final_rf = run(SIMPLE_BRANCH);
        assert_eq!(final_rf.get(R0), 0);
        assert_eq!(final_rf.get(R1), 0);
    }

    #[test]
    fn simple_loop() {
        let mut final_rf = run(SIMPLE_LOOP);
        assert_eq!(final_rf.get(R0), 0);
        assert_eq!(final_rf.get(R1), 1);
        assert_eq!(final_rf.get(R2), 0);
        assert_eq!(final_rf.get(R3), 4);
    }
}
