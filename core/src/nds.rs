use crate::arm7::ARM7;
use crate::arm9::ARM9;
use crate::hw::HW;

pub struct NDS {
    arm7_cycles_ahead: i32, // Measured in 33 MHz ARM7 cycles
    arm7: ARM7,
    arm9: ARM9,
    hw: HW,
}

impl NDS {
    pub fn new(bios7: Vec<u8>, bios9: Vec<u8>, rom: Vec<u8>) -> Self {
        let mut hw = HW::new(bios7, bios9, rom);
        NDS {
            arm7_cycles_ahead: 0,
            arm7: ARM7::new(&mut hw),
            arm9: ARM9::new(&mut hw),
            hw,
        }
    }

    pub fn emulate_frame(&mut self) {
        self.arm7_cycles_ahead += 2 * self.arm9.emulate_instr(&mut self.hw) as i32;
        while self.arm7_cycles_ahead >= 0 {
            self.arm7_cycles_ahead -= self.arm7.emulate_instr(&mut self.hw) as i32;
        }
    }
}
