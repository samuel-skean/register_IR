use std::collections::{HashMap, HashSet};

use crate::{create_label_indices, instructions::Instruction, registers::{LabelName, RegisterName}};

#[derive(PartialEq, Eq, Debug)]
struct InstructionLiveness {
    reads: HashSet<RegisterName>,
    writes: Option<RegisterName>, // Assumption: Instructions only write to at most one register.
    next_instruction_indices: HashSet<usize>,
    out_live: HashSet<RegisterName>,
    in_live: HashSet<RegisterName>,
}

impl InstructionLiveness {
    pub fn from_instruction(index: usize, label_indices: &HashMap<LabelName, usize>, instruction: Instruction) -> Self {
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

fn determine_liveness(program: &[Instruction]) {
    let label_indices = create_label_indices(program);
    let program_liveness_info: Vec<_> = program.into_iter().enumerate().map(|(index, instr)| {
        InstructionLiveness::from_instruction(index, &label_indices, *instr)
    }).collect();

    // NEXT STEP: This sort of idea might be good if we were following branches
    // backwards, but since I don't want to, it really only complicates things.
    // Just iterate through the program forwards until stuff stops changing.
    // NOTE: I'm not sure I'll be able to stop after the first node that doesn't
    // change anything, or after the first traversal that doesn't change
    // anything.
    // NOTE: This set is needlessly expensive, it could literally be a vector of
    // booleans or a bitvector, with the index indicating the instruction index
    // and the boolean indicating its presence. This is because all possibly
    // indices are contiguous (and small).
    let all_instruction_indices: HashSet<_> = (0..program.len()).collect();
    let mut instruction_indices_to_visit = all_instruction_indices.clone();
    let mut curr_instruction_index = 0;
    loop {
        'one_traversal: while instruction_indices_to_visit.len() > 0 {
            instruction_indices_to_visit.remove(&curr_instruction_index);
            println!("Index: {curr_instruction_index}, Instruction: {:?}", program[curr_instruction_index]);

            for &next_instruction_index in program_liveness_info[curr_instruction_index].next_instruction_indices.iter() {
                if instruction_indices_to_visit.contains(&next_instruction_index) {
                    curr_instruction_index = next_instruction_index;
                    continue 'one_traversal;
                }
            }
        }
        instruction_indices_to_visit.clone_from(&all_instruction_indices);
        break;
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_consts::*;

    #[test]
    fn basic_info() {
        let label_liveness = InstructionLiveness::from_instruction(0, &HashMap::new(), Instruction::Label(L0));
        assert_eq!(label_liveness, InstructionLiveness { reads: HashSet::new(), writes: None, next_instruction_indices: HashSet::from([1]), out_live: HashSet::new(), in_live: HashSet::new() });

        let jump_liveness = InstructionLiveness::from_instruction(0, &HashMap::from([(L0, 2)]), Instruction::JumpIfZero(R1, L0));
        assert_eq!(jump_liveness, InstructionLiveness { reads: HashSet::from([R1]), writes: None, next_instruction_indices: HashSet::from([1, 2]), out_live: HashSet::new(), in_live: HashSet::new() });

        let load_imm_liveness = InstructionLiveness::from_instruction(0, &HashMap::new(), Instruction::LoadImmediate(R1, 90));
        assert_eq!(load_imm_liveness, InstructionLiveness { reads: HashSet::new(), writes: Some(R1), next_instruction_indices: HashSet::from([1]), out_live: HashSet::new(), in_live: HashSet::new() });

        let subtract_liveness = InstructionLiveness::from_instruction(0, &HashMap::new(), Instruction::Subtract { assignee: R1, lhs: R2, rhs: R3 });
        assert_eq!(subtract_liveness, InstructionLiveness { reads: HashSet::from([R2, R3]), writes: Some(R1), next_instruction_indices: HashSet::from([1]), out_live: HashSet::new(), in_live: HashSet::new() });
    
    }

    #[test]
    fn basic_straight_line() {
        determine_liveness(BASIC_STRAIGHT_LINE);
    }

    #[test]
    fn simple_branch() {
        determine_liveness(SIMPLE_BRANCH);
    }

    #[test]
    fn simple_loop() {
        determine_liveness(SIMPLE_LOOP);
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