pub const ADDRESS: u8 = 0b1100_0000;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum PLL {
    A = 26,
    B = 34
}

impl PLL {
    pub fn base_addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Multisynth {
    MS0 = 42,
    MS1 = 50,
    MS2 = 58
}

impl Multisynth {
    pub fn base_addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum Register {
    PLL_RESET = 177,
    CLK0 = 16,
    CLK1 = 17,
    CLK2 = 18
}

impl Register {
    pub fn addr(&self) -> u8 {
        *self as u8
    }
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum ClockOutput {
    CLK0 = 16,
    CLK1 = 17,
    CLK2 = 18
}

impl ClockOutput {
    pub fn register(&self) -> Register {
        match self {
            &ClockOutput::CLK0 => Register::CLK0,
            &ClockOutput::CLK1 => Register::CLK1,
            &ClockOutput::CLK2 => Register::CLK2
        }
    }
}
