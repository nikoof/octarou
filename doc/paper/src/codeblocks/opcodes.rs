fn xyn(opcode: u16) -> (usize, usize, u8) {
    (
        ((opcode & 0x0F00) >> 8) as usize,
        ((opcode & 0x00F0) >> 4) as usize,
        (opcode & 0x000F) as u8,
    )
}

fn xnn(opcode: u16) -> (usize, u8) {
    (((opcode & 0x0F00) >> 8) as usize, (opcode & 0x00FF) as u8)
}

fn nnn(opcode: u16) -> usize {
    (opcode & 0x0FFF) as usize
}
