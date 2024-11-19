use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use crate::{
    create_label_indices,
    instructions::Instruction,
    registers::{LabelName, RegisterName},
};

#[derive(PartialEq, Eq, Debug, Clone)]
struct InstructionLivenessBuilder {
    reads_from: HashSet<RegisterName>,
    writes_to: Option<RegisterName>, // Constraint: Instructions only write to at most one register.
    next_instruction_indices: HashSet<usize>,
    out_live: RefCell<HashSet<RegisterName>>,
    in_live: RefCell<HashSet<RegisterName>>,
}

#[derive(PartialEq, Eq, Debug)]
struct InstructionLiveness {
    in_live: HashSet<RegisterName>,
    out_live: HashSet<RegisterName>,
}

impl InstructionLivenessBuilder {
    fn from_instruction(
        index: usize,
        label_indices: &HashMap<LabelName, usize>,
        instruction: Instruction,
        is_last: bool,
    ) -> Self {
        // I was thinking that this code shows that I should be using open-world
        // polymorphism (e.g. interfaces, dyn Trait) instead of enums, but then
        // I realized it only changes where you write the code unless you need
        // to dynamically load new "variants", and that writing the code here is fine.
        let reads = match instruction {
            Instruction::Subtract {
                assignee: _,
                lhs,
                rhs,
            } => HashSet::from([lhs, rhs]),
            Instruction::JumpIfZero(tested, _) => HashSet::from([tested]),
            _ => HashSet::new(),
        };
        let writes = match instruction {
            Instruction::LoadImmediate(assignee, _)
            | Instruction::Subtract {
                assignee,
                lhs: _,
                rhs: _,
            } => Some(assignee),
            _ => None,
        };
        let mut next_instruction_indices = HashSet::new();
        if !is_last {
            next_instruction_indices.insert(index + 1);
        }
        if let Instruction::JumpIfZero(_, target) = instruction {
            next_instruction_indices.insert(label_indices[&target]);
        }

        Self {
            reads_from: reads,
            writes_to: writes,
            next_instruction_indices,
            out_live: HashSet::new().into(),
            in_live: HashSet::new().into(),
        }
    }

    fn build(self) -> InstructionLiveness {
        InstructionLiveness {
            out_live: self.out_live.into_inner(),
            in_live: self.in_live.into_inner(),
        }
    }
}

fn determine_liveness(program: &[Instruction]) -> Vec<InstructionLiveness> {
    let label_indices = create_label_indices(program);
    let program_liveness_info: Vec<_> = program
        .into_iter()
        .enumerate()
        .map(|(index, instr)| {
            InstructionLivenessBuilder::from_instruction(
                index,
                &label_indices,
                *instr,
                index + 1 == program.len(),
            )
        })
        .collect();

    let mut previous_program_liveness_info = program_liveness_info.clone();

    loop {
        for InstructionLivenessBuilder {
            reads_from,
            writes_to,
            next_instruction_indices,
            out_live,
            in_live,
        } in program_liveness_info.iter().rev()
        {
            *in_live.borrow_mut() = reads_from
                .union(
                    &out_live
                        .borrow()
                        .iter()
                        // TODO: This is disgusting:
                        .filter(|&&out| writes_to.filter(|&write| write != out).is_some())
                        .map(ToOwned::to_owned)
                        .collect(),
                )
                .map(ToOwned::to_owned)
                .collect();
            for next_in_live in next_instruction_indices
                .iter()
                .map(|&i| program_liveness_info[i].in_live.borrow())
            {
                let new_out_live = out_live
                    .borrow()
                    .union(&next_in_live)
                    .map(ToOwned::to_owned)
                    .collect();
                *out_live.borrow_mut() = new_out_live;
            }
        }
        if previous_program_liveness_info == program_liveness_info {
            break;
        }
        previous_program_liveness_info.clone_from(&program_liveness_info);
    }
    program_liveness_info
        .into_iter()
        .map(InstructionLivenessBuilder::build)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_consts::*;

    #[test]
    fn basic_info() {
        let label_liveness = InstructionLivenessBuilder::from_instruction(
            0,
            &HashMap::new(),
            Instruction::Label(L0),
            false,
        );
        assert_eq!(
            label_liveness,
            InstructionLivenessBuilder {
                reads_from: HashSet::new(),
                writes_to: None,
                next_instruction_indices: HashSet::from([1]),
                out_live: HashSet::new().into(),
                in_live: HashSet::new().into(),
            }
        );

        let jump_liveness = InstructionLivenessBuilder::from_instruction(
            0,
            &HashMap::from([(L0, 2)]),
            Instruction::JumpIfZero(R1, L0),
            false,
        );
        assert_eq!(
            jump_liveness,
            InstructionLivenessBuilder {
                reads_from: HashSet::from([R1]),
                writes_to: None,
                next_instruction_indices: HashSet::from([1, 2]),
                out_live: HashSet::new().into(),
                in_live: HashSet::new().into(),
            }
        );

        let load_imm_liveness = InstructionLivenessBuilder::from_instruction(
            0,
            &HashMap::new(),
            Instruction::LoadImmediate(R1, 90),
            false,
        );
        assert_eq!(
            load_imm_liveness,
            InstructionLivenessBuilder {
                reads_from: HashSet::new(),
                writes_to: Some(R1),
                next_instruction_indices: HashSet::from([1]),
                out_live: HashSet::new().into(),
                in_live: HashSet::new().into(),
            }
        );

        let subtract_liveness = InstructionLivenessBuilder::from_instruction(
            0,
            &HashMap::new(),
            Instruction::Subtract {
                assignee: R1,
                lhs: R2,
                rhs: R3,
            },
            false,
        );
        assert_eq!(
            subtract_liveness,
            InstructionLivenessBuilder {
                reads_from: HashSet::from([R2, R3]),
                writes_to: Some(R1),
                next_instruction_indices: HashSet::from([1]),
                out_live: HashSet::new().into(),
                in_live: HashSet::new().into(),
            }
        );
    }

    #[test]
    fn basic_straight_line() {
        assert_eq!(
            determine_liveness(BASIC_STRAIGHT_LINE),
            vec![
                InstructionLiveness {
                    in_live: HashSet::from([R0,]),
                    out_live: HashSet::from([R1, R0,]),
                },
                InstructionLiveness {
                    in_live: HashSet::from([R1, R0,]),
                    out_live: HashSet::from([R1,]),
                },
                InstructionLiveness {
                    in_live: HashSet::from([R1,]),
                    out_live: HashSet::from([R2, R1,]),
                },
                InstructionLiveness {
                    in_live: HashSet::from([R1, R2,]),
                    // TODO: Is this okay? Or does it mean code without loops
                    // ends up using no registers?
                    out_live: HashSet::from([]),
                },
            ]
        );
    }

    // TODO: More tests, with branches and loops!
}
