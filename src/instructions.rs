use crate::registers::*;

pub enum Instruction {
    LoadImmediate(RegisterName, u64),
    Subtract { target: RegisterName, lhs: RegisterName, rhs: RegisterName },
    JumpIfZero(RegisterName, LabelName),
    Label(LabelName)
}

impl Instruction {
    pub fn run(&self, rf: &mut RegisterFile) {
        match self {
            &Instruction::LoadImmediate(target, imm) => {
                rf.set(target, imm);
            },
            &Instruction::Subtract { target, lhs, rhs } => {
                let new_value = rf.get(lhs) - rf.get(rhs);
                rf.set(target, new_value);
            },
            Instruction::Label(label_name) => todo!(),
            Instruction::JumpIfZero(register_name, label_name) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{*, Instruction::*};

    const R0: RegisterName = RegisterName::with_value(0);
    const R1: RegisterName = RegisterName::with_value(1);

    #[test]
    fn subtract() {
        let mut rf = RegisterFile::new();
        let program = [
            LoadImmediate(R0, 50),
            LoadImmediate(R1, 30),
            Subtract { target: R1, lhs: R0, rhs: R1 }
        ];

        for instruction in program {
            instruction.run(&mut rf);
        }
        assert_eq!(rf.get(R1), 20);
    }

    #[test]
    fn load_immediate() {
        let mut rf = RegisterFile::new();
        LoadImmediate(R0, 42).run(&mut rf);
        rf.get(R0);
    }
}