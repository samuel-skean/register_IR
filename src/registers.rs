// STRETCH: Let user create fresh registers and labels.
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct RegisterName(usize);

impl RegisterName {
    pub const fn with_value(x: usize) -> Self {
        Self(x)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct LabelName(usize);

impl LabelName {
    pub const fn with_value(x: usize) -> Self {
        Self(x)
    }
}

// TODO: Should we be able to test these for equality? Probably just activate
// all the registers from one in the other before comparing the vectors.
pub struct RegisterFile {
    general_purpose: Vec<u64>,
    pub program_counter: usize,
}

// TODO: Add a register that's always 0.
impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile {
            general_purpose: Vec::new(),
            program_counter: 0,
        }
    }

    // Yes, I know a "get" method taking a mutable reference to self is odd, but
    // there's no actual need for interior mutability here so let's not do it.
    pub fn get(&mut self, reg: RegisterName) -> u64 {
        if self.general_purpose.len() <= reg.0 {
            self.general_purpose.resize(reg.0 + 1, 0);
        }
        self.general_purpose[reg.0]
    }

    pub fn set(&mut self, reg: RegisterName, new_value: u64) {
        self.get(reg);
        self.general_purpose[reg.0] = new_value;
    }
}
