//#![deny(missing_docs)]
#![deny(warnings)]
#![feature(unsize)]
#![no_std]

extern crate cast;
extern crate embedded_hal as hal;

use cast::{f32, u32};
use core::mem;
use hal::blocking::i2c::{Write, WriteRead};

mod si5351;

/// Si5351 driver
pub struct Si5351<I2C> {
    i2c: I2C,
    address: u8,
    xtal_freq: u32,
}

impl<I2C, E> Si5351<I2C>
    where
        I2C: WriteRead<Error=E> + Write<Error=E>,
{
    /// Creates a new driver from a I2C peripheral
    pub fn new(i2c: I2C, address_bit: bool, xtal_freq: u32) -> Result<Self, E> {
        let si5351 = Si5351 {
            i2c,
            address: si5351::ADDRESS | if address_bit { 1 } else { 0 },
            xtal_freq,
        };

        Ok(si5351)
    }

    pub fn set_frequency(&mut self, freq: u32) -> Result<(), E> {
        let pll_freq: u32;
        let l: u32;
        let f: f32;
        let mult: u8;
        let num: u32;
        let denom: u32;
        let divider: u32;

        divider = match 900000000 / freq {
            d if d % 2 == 0 => d,
            d => d - 1
        };

        pll_freq = divider * freq;

        mult = (pll_freq / self.xtal_freq) as u8;
        l = pll_freq % self.xtal_freq;
        f = (l as f32) * (1048575 as f32) / (self.xtal_freq as f32);
        num = f as u32;
        denom = 1048575;

        self.setup_pll(si5351::PLL::A, mult, num, denom)?;
        self.setup_multisynth(si5351::Multisynth::MS0, divider, 0)?;
        self.reset_pll(si5351::PLL::A)?;
        self.enable_clock(si5351::ClockOutput::CLK0)?;

        Ok(())
    }

    fn setup_pll(&mut self, pll: si5351::PLL, mult: u8, num: u32, denom: u32) -> Result<(), E> {
        let p1: u32;
        let p2: u32;
        let p3: u32;
        let ratio = (128f32 * (f32(num) / f32(denom))) as u32;

        p1 = u32(128u32 * mult as u32 + ratio - 512);
        p2 = u32(128 * num - denom * ratio);
        p3 = denom;

        self.write_pll_register(pll, 0, ((p3 & 0x0000FF00) >> 8) as u8)?;
        self.write_pll_register(pll, 1, p3 as u8)?;
        self.write_pll_register(pll, 2, ((p1 & 0x00030000) >> 16) as u8)?;
        self.write_pll_register(pll, 3, ((p1 & 0x0000FF00) >> 8) as u8)?;
        self.write_pll_register(pll, 4, p1 as u8)?;
        self.write_pll_register(pll, 5, (((p3 & 0x000F0000) >> 12) | ((p2 & 0x000F0000) >> 16)) as u8)?;
        self.write_pll_register(pll, 6, ((p2 & 0x0000FF00) >> 8) as u8)?;
        self.write_pll_register(pll, 7, p2 as u8)?;

        Ok(())
    }

    fn setup_multisynth(&mut self, synth: si5351::Multisynth, divider: u32, r_div: u8) -> Result<(), E> {
        let p1: u32;
        let p2: u32;
        let p3: u32;

        p1 = 128 * divider - 512;
        p2 = 0;
        p3 = 1;

        self.write_synth_register(synth, 0, ((p3 & 0x0000FF00) >> 8) as u8)?;
        self.write_synth_register(synth, 1, p3 as u8)?;
        self.write_synth_register(synth, 2, ((p1 & 0x00030000) >> 16) as u8 | r_div)?;
        self.write_synth_register(synth, 3, ((p1 & 0x0000FF00) >> 8) as u8)?;
        self.write_synth_register(synth, 4, p1 as u8)?;
        self.write_synth_register(synth, 5, (((p3 & 0x000F0000) >> 12) | ((p2 & 0x000F0000) >> 16)) as u8)?;
        self.write_synth_register(synth, 6, ((p2 & 0x0000FF00) >> 8) as u8)?;
        self.write_synth_register(synth, 7, p2 as u8)?;

        Ok(())
    }

    fn reset_pll(&mut self, pll: si5351::PLL) -> Result<(), E> {
        self.write_register(si5351::Register::PLL_RESET,
                            match pll {
                                si5351::PLL::A => 0b0010_0000,
                                si5351::PLL::B => 0b1000_0000
                            })?;

        Ok(())
    }

    fn enable_clock(&mut self, clock: si5351::ClockOutput) -> Result<(), E> {
        self.write_register(clock.register(), 0x4f)
    }

    #[allow(unused)]
    fn read_register(&mut self, reg: si5351::Register) -> Result<u8, E> {
        let mut buffer: [u8; 1] = unsafe { mem::uninitialized() };
        self.i2c.write_read(self.address, &[reg.addr()], &mut buffer)?;
        Ok(buffer[0])
    }

    fn write_register(&mut self, reg: si5351::Register, byte: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[reg.addr(), byte])
    }

    fn write_pll_register(&mut self, pll: si5351::PLL, reg: u8, byte: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[pll.base_addr() + reg, byte])
    }

    fn write_synth_register(&mut self, pll: si5351::Multisynth, reg: u8, byte: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[pll.base_addr() + reg, byte])
    }
}
