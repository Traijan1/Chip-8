use crate::emulator::{chip8, cpu::Cpu, error::EmulatorError::{self}, framebuffer::Framebuffer, instruction::Instruction, keypad::Keypad, memory::{FONT_START, Memory}};

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    pub framebuffer: Framebuffer,
    pub keypad: Keypad
}

impl Chip8 {
    pub fn reset(&mut self) {
        self.cpu = Cpu::new();
        self.framebuffer.clear_buffer();
        self.memory = Memory::new();
        self.keypad = Keypad::new();
    }

    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            framebuffer: Framebuffer::new(),
            keypad: Keypad::new()
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), EmulatorError> {
        self.reset();
        self.memory.load_rom(rom)
    }

    fn fetch_instruction(&mut self) -> Result<Instruction, EmulatorError> {
        let pc = self.cpu.program_counter;

        let first_byte = self.memory.read(pc)? as u16;
        let second_byte = self.memory.read(pc + 1)? as u16;
        
        let opcode = first_byte << 8 | second_byte;

        // Advance the PC only after a successful decode, so on error it still points to the faulty instruction
        let instruction = Instruction::try_from(opcode)?;

        self.cpu.program_counter += 2;

        Ok(instruction)
    }

    pub fn step(&mut self) -> Result<(), EmulatorError> {
        let instruction = self.fetch_instruction()?;
        
        match instruction {
            Instruction::Clear => self.framebuffer.clear_buffer(),
            Instruction::SetRegister(x, nn) => self.cpu.set_v(x,nn),
            Instruction::SetIndex(nnn) => self.cpu.index_register = nnn,
            Instruction::Draw(x, y, n) => {
                let to_draw = self.memory.read_range(self.cpu.index_register, n)?;
                let collision = self.framebuffer.draw(self.cpu.v(x), self.cpu.v(y), to_draw);
                self.cpu.v[0xF] = collision;
            }
            Instruction::Jump(nnn) => self.cpu.program_counter = nnn,
            Instruction::JumpWithOffset(nnn) => self.cpu.program_counter = nnn + self.cpu.v(0) as u16,
            Instruction::AddToRegister(x, nn) => {
                let register_value = self.cpu.v(x);
                self.cpu.set_v(x, register_value.wrapping_add(nn));
            },
            Instruction::AddToIndex(x) => {
                let index_register = self.cpu.index_register;
                self.cpu.index_register = index_register.wrapping_add(self.cpu.v(x) as u16);
            },
            Instruction::SkipIfEqual(x, nn) => {
                if self.cpu.v(x) == nn {
                    self.cpu.program_counter += 2;
                }
            },
            Instruction::SkipIfNotEqual(x, nn) => {
                if self.cpu.v(x) != nn {
                    self.cpu.program_counter += 2;
                }
            },
            Instruction::SkipIfRegistersEqual(x, y) => {
                if self.cpu.v(x) == self.cpu.v(y) {
                    self.cpu.program_counter += 2;
                }
            },
            Instruction::SkipIfRegistersNotEqual(x, y) => {
                if self.cpu.v(x) != self.cpu.v(y) {
                    self.cpu.program_counter += 2;
                }
            },
            Instruction::CopyRegister(x, y) => self.cpu.set_v(x, self.cpu.v(y)),
            Instruction::BitwiseOr(x, y) => {
                let x_value = self.cpu.v(x);
                let y_value = self.cpu.v(y);

                self.cpu.set_v(x, x_value | y_value);
                self.cpu.set_v(0xF, 0);
            },
            Instruction::BitwiseAnd(x, y) => {
                let x_value = self.cpu.v(x);
                let y_value = self.cpu.v(y);

                self.cpu.set_v(x, x_value & y_value);
                self.cpu.set_v(0xF, 0);
            },
            Instruction::BitwiseXor(x, y) => {
                let x_value = self.cpu.v(x);
                let y_value = self.cpu.v(y);

                self.cpu.set_v(x, x_value ^ y_value);
                self.cpu.set_v(0xF, 0);
            },
            Instruction::Addition(x, y) => {
                let x_value = self.cpu.v(x);
                let y_value = self.cpu.v(y);

                let (result, carry) = x_value.overflowing_add(y_value);

                self.cpu.set_v(x, result);
                self.cpu.set_v(0xF, carry as u8);
            },
            Instruction::Subtraction(x, y) => {
                let x_value = self.cpu.v(x);
                let y_value = self.cpu.v(y);

                let (result, borrow) = x_value.overflowing_sub(y_value);

                self.cpu.set_v(x, result);
                self.cpu.set_v(0xF, !borrow as u8);
            },
            Instruction::RightShift(x, _) => {
                let x_value = self.cpu.v(x);
                let f_value = x_value & 1;

                self.cpu.set_v(x, x_value >> 1);
                self.cpu.set_v(0xF, f_value);
            },
            Instruction::ReverseSubtraction(x, y) => {
                let x_value = self.cpu.v(x);
                let y_value = self.cpu.v(y);

                let (result, borrow) = y_value.overflowing_sub(x_value);

                self.cpu.set_v(x, result);
                self.cpu.set_v(0xF, !borrow as u8);
            },
            Instruction::LeftShift(x, _) => {
                let x_value = self.cpu.v(x);
                let f_value = x_value >> 7;

                self.cpu.set_v(x, x_value << 1);
                self.cpu.set_v(0xF, f_value);
            },
            Instruction::RandomWithBitwiseAnd(x, nn) => {
                let rng = rand::random::<u8>();
                self.cpu.set_v(x, rng & nn);
            },
            Instruction::GetDelayTimer(x) => self.cpu.set_v(x, self.cpu.delay_timer),
            Instruction::SetDelayTimer(x) => self.cpu.delay_timer = self.cpu.v(x),
            Instruction::SetSoundTimer(x) => self.cpu.sound_timer = self.cpu.v(x),
            Instruction::StoreBcd(x) => {
                let index_register = self.cpu.index_register;
                let x = self.cpu.v(x);

                let hundreds = x / 100;
                let tens = x / 10 % 10;
                let ones = x % 10;

                self.memory.write(index_register, hundreds)?;
                self.memory.write(index_register.wrapping_add(1), tens)?;
                self.memory.write(index_register.wrapping_add(2), ones)?;
            },
            Instruction::StoreRegisters(x) => {
                let index_register = self.cpu.index_register;
                
                for i in 0..=x {
                    let register_value = self.cpu.v(i);
                    self.memory.write(index_register.wrapping_add(i as u16), register_value)?;
                }

                self.cpu.index_register += x as u16 + 1;
            },
            Instruction::LoadRegisters(x) => {
                let index_register = self.cpu.index_register;
                
                for i in 0..=x {
                    let memory_value = self.memory.read(index_register.wrapping_add(i as u16))?;
                    self.cpu.set_v(i, memory_value);
                }

                self.cpu.index_register += x as u16 + 1;
            },
            Instruction::Return => self.cpu.program_counter = self.cpu.pop()?,
            Instruction::Call(nnn) => {
                self.cpu.push(self.cpu.program_counter)?;
                self.cpu.program_counter = nnn;
            },
            Instruction::SkipIfKeyDown(x) => {
                let vx = self.cpu.v(x);
                
                if self.keypad.get_key(vx) {
                    self.cpu.program_counter += 2;
                }
            },
            Instruction::SkipIfKeyUp(x) => {
                let vx = self.cpu.v(x);
                
                if !self.keypad.get_key(vx) {
                    self.cpu.program_counter += 2;
                }
            },
            Instruction::WaitKeyPress(x) => {
                let was_pressed = self.keypad.was_key_pressed();

                match was_pressed {
                    Some(value) => self.cpu.set_v(x, value),
                    None => self.cpu.program_counter -= 2
                };
            },
            Instruction::SetIndexToFont(x) => {
                let vx = self.cpu.v(x) & 0xF;
                self.cpu.index_register = (FONT_START as u16 + vx as u16) * 5; 
            }
        }

        Ok(())
    }

    pub fn tick_timers(&mut self) {
        self.cpu.delay_timer = self.cpu.delay_timer.saturating_sub(1);
        self.cpu.sound_timer = self.cpu.sound_timer.saturating_sub(1);
    }

    pub fn sound_timer(&self) -> u8 {
        self.cpu.sound_timer
    }
}