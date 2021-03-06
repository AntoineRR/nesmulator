// Represents the CPU bus of the NES

// ===== IMPORTS =====

use std::sync::{Arc, Mutex};

use cartridge::mapper::Mapper;

use crate::{cartridge, controllers::Controller, ppu::ppu::PPU};

// ===== CONSTANTS =====

pub const STACK_OFFSET: u16 = 0x100;

// ===== BUS STRUCT =====

pub struct Bus {
    pub data: [u8;0x10000],
    pub o_p_mapper: Option<Box<dyn Mapper>>,
    pub p_ppu: Arc<Mutex<PPU>>,
    
    pub controllers: [Controller;2]
}

impl Bus {
    pub fn new(p_ppu: Arc<Mutex<PPU>>) -> Self {
        Bus {
            data: [0;0x10000], // 64KB of ram
            o_p_mapper:None,
            p_ppu,
            
            controllers: [Controller::new();2]
        }
    }

    // Reads data from the bus at the specified address
    pub fn read(&mut self, address: u16) -> u8 {
        let value: u8;
        match address {
            // 0x0000 - 0x07FF / 2KB CPU RAM
            0x0000..=0x7FF => value = self.data[address as usize],
            // 0x0800 - 0x1FFF / CPU RAM Mirrors
            0x0800..=0x1FFF => value = self.data[(address & 0x07FF) as usize],
            // 0x2000 - 0x2007 / NES PPU Registers
            0x2000..=0x2007 => value = self.p_ppu.lock().unwrap().read_register(address),
            // 0x2008 - 0x3FFF / NES PPU Registers Mirrors
            0x2008..=0x3FFF => value = self.p_ppu.lock().unwrap().read_register(address & 0x2007),
            // 0x4000 - 0x4013 / NES APU I/O Registers
            0x4000..=0x4013 => value = self.data[address as usize],
            // 0x4014 / NES PPU Register
            0x4014 => value = self.p_ppu.lock().unwrap().read_register(address),
            // 0x4015 / NES APU Register
            0x4015 => value = self.data[address as usize],
            // 0x4016 / First controller
            0x4016 => value = self.controllers[0].check_shifter(),
            // 0x4017 / Second controller
            0x4017 => value = self.controllers[1].check_shifter(),
            // 0x4018 - 0x4020 / I/O Refisters
            0x4018..=0x4020 => value = self.data[address as usize],
            // 0x4021 - 0xFFFF / Handled by the mapper
            0x4021..=0xFFFF => value = self.o_p_mapper.as_ref().unwrap().prg_rom_read(address)
        }
        value
    }

    // Used for debugging
    // Some normal reads may change the state of some elements (ex: 2002 for PPU)
    // Use this method to avoid it
    #[allow(dead_code)]
    pub fn read_only(&self, address: u16) -> u8 {
        let value: u8;
        match address {
            // 0x0000 - 0x07FF / 2KB CPU RAM
            0x0000..=0x7FF => value = self.data[address as usize],
            // 0x0800 - 0x1FFF / CPU RAM Mirrors
            0x0800..=0x1FFF => value = self.data[(address & 0x07FF) as usize],
            // 0x2000 - 0x2007 / NES PPU Registers
            0x2000..=0x2007 => value = self.p_ppu.lock().unwrap().registers.read_register_without_modification(address),
            // 0x2008 - 0x3FFF / NES PPU Registers Mirrors
            0x2008..=0x3FFF => value = self.p_ppu.lock().unwrap().registers.read_register_without_modification(address & 0x2007),
            // 0x4000 - 0x4013 / NES APU I/O Registers
            0x4000..=0x4013 => value = self.data[address as usize],
            // 0x4014 / NES PPU Register
            0x4014 => value = self.p_ppu.lock().unwrap().registers.read_register_without_modification(address),
            // 0x4015 / NES APU Register
            0x4015 => value = self.data[address as usize],
            // 0x4016 / First controller
            0x4016 => value = self.data[address as usize],
            // 0x4017 / Second controller
            0x4017 => value = self.data[address as usize],
            // 0x4018 - 0x4020 / I/O Refisters
            0x4018..=0x4020 => value = self.data[address as usize],
            // 0x4021 - 0xFFFF / Handled by the mapper
            0x4021..=0xFFFF => value = self.o_p_mapper.as_ref().unwrap().prg_rom_read(address)
        }
        value
    }

    // Writes data to the bus at the specified address
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            // 0x0000 - 0x07FF / 2KB CPU RAM
            0x0000..=0x7FF => self.data[address as usize] = value,
            // 0x0800 - 0x1FFF / CPU RAM Mirrors
            0x0800..=0x1FFF => self.data[(address & 0x07FF) as usize] = value,
            // 0x2000 - 0x2007 / NES PPU Registers
            0x2000..=0x2007 => self.p_ppu.lock().unwrap().write_register(address, value),
            // 0x2008 - 0x3FFF / NES PPU Registers Mirrors
            0x2008..=0x3FFF => self.p_ppu.lock().unwrap().write_register(address & 0x2007, value),
            // 0x4000 - 0x4013 / NES APU I/O Registers
            0x4000..=0x4013 => self.data[address as usize] = value,
            // 0x4014 / NES PPU Register
            0x4014 => self.p_ppu.lock().unwrap().write_register(address, value),
            // 0x4015 / NES APU Register
            0x4015 => self.data[address as usize] = value,
            // 0x4016 / First controller
            0x4016 => {
                if (value & 0x01) > 0 {
                    self.controllers[0].update_shifter();
                }
            }
            // 0x4017 / Second controller
            0x4017 => {
                if (value & 0x01) > 0 {
                    self.controllers[1].update_shifter();
                }
            }
            // 0x4018 - 0x4020 / I/O Refisters
            0x4018..=0x4020 => self.data[address as usize] = value,
            // 0x4021 - 0xFFFF / Handled by the mapper
            0x4021..=0xFFFF => self.o_p_mapper.as_mut().unwrap().prg_rom_write(address, value)
        }
    }
}