use crate::registers::*;

pub enum Instruction {
    LoadImmediate(RegisterName, u64),
    Subtract { target: RegisterName, lhs: RegisterName, rhs: RegisterName },
    JumpIfZero(RegisterName, LabelName),
    Label(LabelName)
}

impl Instruction {
    pub fn interpret(&self, rf: &mut RegisterFile) {
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

    #[test]
    fn subtract() {
        let mut rf = RegisterFile::new();
        let reg0 = RegisterName::with_value(0);
        let reg1 = RegisterName::with_value(1);
        let program = [
            LoadImmediate(reg0, 50),
            LoadImmediate(reg1, 30),
            Subtract { target: reg1, lhs: reg0, rhs: reg1 }
        ];

        for instruction in program {
            instruction.interpret(&mut rf);
        }
        assert_eq!(rf.get(reg1), 20);
    }
}