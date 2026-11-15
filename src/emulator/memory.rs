use crate::emulator::error::EmulatorError::{self, AddressOutOfBounds};

pub const PROGRAM_START: usize = 0x200;
const MEMORY_SIZE: usize = 0x1000;
const PROGRAM_SPACE: usize = MEMORY_SIZE - PROGRAM_START;

pub(super) const FONT_START: usize = 0x0;
const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];


pub(super) struct Memory {
    ram: [u8; MEMORY_SIZE]
}

impl Memory {
    pub(super) fn new() -> Self {
        let mut ram = [0; MEMORY_SIZE];
        ram[FONT_START..FONT_START + FONT.len()].copy_from_slice(&FONT);

        Self {
            ram
        }
    }

    pub(super) fn load_rom(&mut self, rom: &[u8]) -> Result<(), EmulatorError> {
        if rom.len() > PROGRAM_SPACE {
            return Err(EmulatorError::RomTooLarge(rom.len(), PROGRAM_SPACE));
        }

        self.ram[PROGRAM_START..PROGRAM_START + rom.len()].copy_from_slice(rom);

        Ok(())
    }

    pub(super) fn read(&self, address: u16) -> Result<u8, EmulatorError> {
        self.ram.get(address as usize).copied().ok_or(AddressOutOfBounds(address))
    }

    pub(super) fn read_range(&self, address_start: u16, length: u8) -> Result<&[u8], EmulatorError> {
        let end = address_start as usize + length as usize;
        self.ram.get(address_start as usize..end).ok_or(AddressOutOfBounds(address_start))
    }

    pub(super) fn write(&mut self, address: u16, value: u8) -> Result<(), EmulatorError> {
        let entry = self.ram.get_mut(address as usize).ok_or(AddressOutOfBounds(address))?;
        *entry = value;

        Ok(())
    }
}