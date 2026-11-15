use crate::emulator::{error::EmulatorError, instruction::Instruction::SkipIfRegistersNotEqual};

pub enum Instruction {
    Clear,
    Return,
    Jump(u16),
    Call(u16),
    SkipIfEqual(u8, u8),
    SkipIfNotEqual(u8, u8),
    SkipIfRegistersEqual(u8, u8),
    SkipIfRegistersNotEqual(u8, u8),
    SetRegister(u8, u8),
    AddToRegister(u8, u8),
    CopyRegister(u8, u8),
    SetIndex(u16),
    Draw(u8, u8, u8),
    StoreBcd(u8),
    StoreRegisters(u8),
    LoadRegisters(u8),
    BitwiseOr(u8, u8),
    BitwiseAnd(u8, u8),
    BitwiseXor(u8, u8),
    Addition(u8, u8),
    Subtraction(u8, u8),
    RightShift(u8, u8),
    ReverseSubtraction(u8, u8),
    LeftShift(u8, u8),
    JumpWithOffset(u16),
    RandomWithBitwiseAnd(u8, u8),
    GetDelayTimer(u8),
    SetDelayTimer(u8),
    SetSoundTimer(u8),
    AddToIndex(u8),
    SkipIfKeyDown(u8),
    SkipIfKeyUp(u8),
    WaitKeyPress(u8),
    SetIndexToFont(u8),
}

impl TryFrom<u16> for Instruction {
    type Error = EmulatorError;

    fn try_from(opcode: u16) -> Result<Self, Self::Error> {
        let first_nibble = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;  
        

        match (first_nibble, x, y, n) {
            (0x0, 0x0, 0xE, 0x0) => Ok(Instruction::Clear),
            (0x0, 0x0, 0xE, 0xE) => Ok(Instruction::Return),
            (0x1, _, _, _) => Ok(Instruction::Jump(nnn)),
            (0x2, _, _, _) => Ok(Instruction::Call(nnn)),
            (0x3, _, _, _) => Ok(Instruction::SkipIfEqual(x, nn)),
            (0x4, _, _, _) => Ok(Instruction::SkipIfNotEqual(x, nn)),
            (0x5, _, _, 0) => Ok(Instruction::SkipIfRegistersEqual(x, y)),
            (0x6, _, _, _) => Ok(Instruction::SetRegister(x, nn)),
            (0x7, _, _, _) => Ok(Instruction::AddToRegister(x, nn)),
            (0x8, _, _, 0) => Ok(Instruction::CopyRegister(x, y)),
            (0x8, _, _, 1) => Ok(Instruction::BitwiseOr(x, y)),
            (0x8, _, _, 2) => Ok(Instruction::BitwiseAnd(x, y)),
            (0x8, _, _, 3) => Ok(Instruction::BitwiseXor(x, y)),
            (0x8, _, _, 4) => Ok(Instruction::Addition(x, y)),
            (0x8, _, _, 5) => Ok(Instruction::Subtraction(x, y)),
            (0x8, _, _, 6) => Ok(Instruction::RightShift(x, y)),
            (0x8, _, _, 7) => Ok(Instruction::ReverseSubtraction(x, y)),
            (0x8, _, _, 0xE) => Ok(Instruction::LeftShift(x, y)),
            (0x9, _, _, 0) => Ok(Instruction::SkipIfRegistersNotEqual(x, y)),
            (0xA, _, _, _) => Ok(Instruction::SetIndex(nnn)),
            (0xB, _, _, _) => Ok(Instruction::JumpWithOffset(nnn)),
            (0xC, _, _, _) => Ok(Instruction::RandomWithBitwiseAnd(x, nn)),
            (0xD, _, _, _) => Ok(Instruction::Draw(x, y, n)),
            (0xE, _, 9, 0xE) => Ok(Instruction::SkipIfKeyDown(x)),
            (0xE, _, 0xA, 1) => Ok(Instruction::SkipIfKeyUp(x)),
            (0xF, _, 0, 7) => Ok(Instruction::GetDelayTimer(x)),
            (0xF, _, 0, 0xA) => Ok(Instruction::WaitKeyPress(x)),
            (0xF, _, 1, 5) => Ok(Instruction::SetDelayTimer(x)),
            (0xF, _, 1, 8) => Ok(Instruction::SetSoundTimer(x)),
            (0xF, _, 1, 0xE) => Ok(Instruction::AddToIndex(x)),
            (0xF, _, 2, 9) => Ok(Instruction::SetIndexToFont(x)),
            (0xF, _, 3, 3) => Ok(Instruction::StoreBcd(x)),
            (0xF, _, 5, 5) => Ok(Instruction::StoreRegisters(x)),
            (0xF, _, 6, 5) => Ok(Instruction::LoadRegisters(x)),
            _ => return Err(EmulatorError::OpCodeDoesNotExist(opcode))
        }
    }
}