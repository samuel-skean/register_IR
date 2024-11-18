use std::collections::{HashMap, HashSet};

use crate::{instructions::Instruction, registers::{LabelName, RegisterName}};

#[derive(PartialEq, Eq, Debug)]
struct InstructionLiveness {
    reads: HashSet<RegisterName>,
    writes: Option<RegisterName>, // Assumption: Instructions only write to at most one register.
    next_instruction_indices: HashSet<usize>,
    out_live: HashSet<RegisterName>,
    in_live: HashSet<RegisterName>,
}

impl InstructionLiveness {
    pub fn from_instruction(index: usize, label_indices: HashMap<LabelName, usize>, instruction: Instruction) -> Self {
        // I was thinking that this code shows that I should be using open-world
        // polymorphism (e.g. interfaces, dyn Trait) instead of enums, but then
        // I realized it only changes where you write the code unless you need
        // to dynamically load new "variants", and that writing the code here is fine.
        let reads = match instruction {
            Instruction::Subtract { assignee: _, lhs, rhs } => HashSet::from([lhs, rhs]),
            Instruction::JumpIfZero(tested, _) => HashSet::from([tested]),
            _ => HashSet::new(),
        };
        let writes = match instruction {
            Instruction::LoadImmediate(assignee, _)
            | Instruction::Subtract { assignee, lhs: _, rhs: _ } => Some(assignee),
            _ => None,
        };
        let mut next_instruction_indices = HashSet::from([index + 1]);
        if let Instruction::JumpIfZero(_, target) = instruction {
            next_instruction_indices.insert(label_indices[&target]);
        }
        
        Self { reads, writes, next_instruction_indices, out_live: HashSet::new(), in_live: HashSet::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_consts::*;

    #[test]
    fn basic_info() {
        let label_liveness = InstructionLiveness::from_instruction(0, HashMap::new(), Instruction::Label(L0));
        assert_eq!(label_liveness, InstructionLiveness { reads: HashSet::new(), writes: None, next_instruction_indices: HashSet::from([1]), out_live: HashSet::new(), in_live: HashSet::new() });

        let jump_liveness = InstructionLiveness::from_instruction(0, HashMap::from([(L0, 2)]), Instruction::JumpIfZero(R1, L0));
        assert_eq!(jump_liveness, InstructionLiveness { reads: HashSet::from([R1]), writes: None, next_instruction_indices: HashSet::from([1, 2]), out_live: HashSet::new(), in_live: HashSet::new() });

        let load_imm_liveness = InstructionLiveness::from_instruction(0, HashMap::new(), Instruction::LoadImmediate(R1, 90));
        assert_eq!(load_imm_liveness, InstructionLiveness { reads: HashSet::new(), writes: Some(R1), next_instruction_indices: HashSet::from([1]), out_live: HashSet::new(), in_live: HashSet::new() });

        let subtract_liveness = InstructionLiveness::from_instruction(0, HashMap::new(), Instruction::Subtract { assignee: R1, lhs: R2, rhs: R3 });
        assert_eq!(subtract_liveness, InstructionLiveness { reads: HashSet::from([R2, R3]), writes: Some(R1), next_instruction_indices: HashSet::from([1]), out_live: HashSet::new(), in_live: HashSet::new() });
    
    }
}

// TODO: Actually do liveness analysis - run the fixpoint algorithm. To
// do that, I think you'll need to have a limited size ring buffer of previously
// visited instructions, so you don't get stuck in a rut looking at the same
// instructions multiple times, but you can still revisit them after full
// traversals. Probably just build it out of a VecDeque and use truncate after
// every insert.
// Or, maybe you need to fully visit every instruction and then start fresh.
// That seems more obviously correct.