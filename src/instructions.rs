use std::collections::HashMap;

use crate::registers::*;

pub enum Instruction {
    LoadImmediate(RegisterName, u64),
    Subtract { assignee: RegisterName, lhs: RegisterName, rhs: RegisterName },
    JumpIfZero(RegisterName, LabelName),
    Label(LabelName)
}

impl Instruction {
    pub fn run(&self, rf: &mut RegisterFile, label_indices: &HashMap<LabelName, usize>) {
        match self {
            &Instruction::LoadImmediate(assignee, imm) => {
                rf.set(assignee, imm);
                rf.program_counter += 1;
            },
            &Instruction::Subtract { assignee, lhs, rhs } => {
                let new_value = rf.get(lhs) - rf.get(rhs);
                rf.set(assignee, new_value);
                rf.program_counter += 1;
            },
            Instruction::Label(_) => {
                rf.program_counter += 1;
            },
            Instruction::JumpIfZero(tested, target) => {
                if rf.get(*tested) == 0 {
                    rf.program_counter = label_indices[target];
                } else {
                    rf.program_counter += 1;
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{*, Instruction::*};
    use crate::test_consts::*;

    #[test]
    fn subtract() {
        let mut rf = RegisterFile::new();
        let program = [
            LoadImmediate(R0, 50),
            LoadImmediate(R1, 30),
            Subtract { assignee: R1, lhs: R0, rhs: R1 }
        ];

        for instruction in program {
            instruction.run(&mut rf, &HashMap::new());
        }
        assert_eq!(rf.get(R1), 20);
    }

    #[test]
    fn load_immediate() {
        let mut rf = RegisterFile::new();
        LoadImmediate(R0, 42).run(&mut rf, &HashMap::new());
        rf.get(R0);
    }
    
    #[test]
    fn jump_if_zero() {
        let mut rf = RegisterFile::new();
        let label_indices = HashMap::from([(L0, 44)]);

        // Jump Not Taken:
        LoadImmediate(R1, 20).run(&mut rf, &label_indices);
        JumpIfZero(R1, L0).run(&mut rf, &label_indices);
        assert_eq!(rf.program_counter, 2);

        // Jump Taken:
        LoadImmediate(R2, 0).run(&mut rf, &label_indices);
        JumpIfZero(R2, L0).run(&mut rf, &label_indices);
        assert_eq!(rf.program_counter, 44);

    }
}