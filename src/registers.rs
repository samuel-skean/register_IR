// STRETCH: Let user create fresh registers and labels.
#[derive(Clone, Copy)]
pub struct RegisterName(usize);

impl RegisterName {
    pub fn with_value(x: usize) -> Self {
        Self(x)
    }
}

pub struct LabelName(usize);

impl LabelName {
    pub fn with_value(x: usize) -> Self {
        Self(x)
    }
}

pub struct RegisterFile(Vec<u64>);

// TODO: Add special registers, including a 0 register and a program counter.
impl RegisterFile {
    pub fn new() -> Self {
        RegisterFile(Vec::new())
    }

    // Yes, I know a "get" method taking a mutable reference to self is odd, but
    // there's no actual need for interior mutability here so let's not do it.
    pub fn get(&mut self, reg: RegisterName) -> u64 {
        if self.0.len() <= reg.0 {
            self.0.resize(reg.0 + 1, 0);
        }
        self.0[reg.0]
    }

    pub fn set(&mut self, reg: RegisterName, new_value: u64) {
        self.get(reg);
        self.0[reg.0] = new_value;
    }
}