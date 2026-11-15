use crate::emulator::{error::EmulatorError, memory::PROGRAM_START};

const REGISTER_SIZE: usize = 16;

pub(super) struct Cpu {
    pub(super) v: [u8; REGISTER_SIZE],
    pub(super) index_register: u16,
    pub(super) delay_timer: u8,
    pub(super) sound_timer: u8,
    pub(super) program_counter: u16,
    stack_pointer: usize,
    stack: [u16; 16]
}

impl Cpu {
    pub(super) fn new() -> Self {
        Self {
            v: [0; 16],
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: PROGRAM_START as u16,
            stack_pointer: 0,
            stack: [0; 16]
        }
    }

    pub(super) fn v(&self, address: u8) -> u8 {
        self.v[address as usize]
    } 

    pub(super) fn set_v(&mut self, address: u8, value: u8) {
        self.v[address as usize] = value;
    }

    pub(super) fn push(&mut self, value: u16) -> Result<(), EmulatorError> {
        let entry = self.stack.get_mut(self.stack_pointer).ok_or(EmulatorError::StackOverflow)?;
        *entry = value;

        self.stack_pointer += 1;

        Ok(())
    }  

    pub(super) fn pop(&mut self) -> Result<u16, EmulatorError> {
        if self.stack_pointer == 0 {
            return Err(EmulatorError::StackUnderflow);
        }

        self.stack_pointer -= 1;
        let entry = self.stack[self.stack_pointer];

        Ok(entry)
    }  
}