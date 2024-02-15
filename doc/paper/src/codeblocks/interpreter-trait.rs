pub trait Interpreter {
    fn display(&self) -> Vec<&[u8]>;
    fn is_beeping(&self) -> bool;

    fn update_timers(&mut self);
    fn next_instruction(&mut self) -> Result<Instruction, InterpreterError>;
    fn execute_instruction(
        &mut self,
        instruction: Instruction,
        keys_pressed: &[bool; 16],
        keys_released: &[bool; 16],
    ) -> Result<(), InterpreterError>;

    fn tick(
        &mut self,
        keys_down: &[bool; 16],
        keys_released: &[bool; 16],
        speed: u64,
    ) -> Result<(), InterpreterError> {
        // auto-implementation
    }
}
