fn next_instruction(&mut self) -> Result<Instruction, InterpreterError> {
    let opcode = u16::from_be_bytes(
        self.memory
            .get(self.pc..self.pc + 2)
            .ok_or(InterpreterError::OutOfMemory)?
            .try_into()
            .expect("Slice should always have length 2"),
    );

    self.pc += 2;
    Instruction::new(opcode).ok_or(InterpreterError::UnknownOpcode {
        opcode,
        address: self.pc,
    })
}
