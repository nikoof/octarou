use anyhow::Result;
use std::time;

use super::instruction::Instruction;

pub trait Interpreter {
    fn speed(&self) -> u64;
    fn display(&self) -> Vec<&[u8]>;

    fn update_timers(&mut self);
    fn next_instruction(&mut self) -> Result<Instruction>;
    fn execute_instruction<F: Fn(u8) -> bool, G: Fn() -> Option<u8>>(
        &mut self,
        instruction: Instruction,
        is_key_pressed: F,
        get_key: G,
    ) -> Result<()>;

    fn tick<F, G>(&mut self, is_key_pressed: F, get_key: G) -> Result<()>
    where
        F: Fn(u8) -> bool,
        G: Fn() -> Option<u8>,
    {
        let timer_cycle_duration = time::Duration::from_nanos(1_000_000_000 / 60);
        let cpu_cycle_duration = time::Duration::from_nanos(1_000_000_000 / self.speed());

        let now = time::Instant::now();
        let mut total_elapsed = time::Duration::from_secs(0);

        self.update_timers();

        'cpu: loop {
            match self.next_instruction() {
                Ok(next_instruction) => {
                    self.execute_instruction(next_instruction, &is_key_pressed, &get_key)?
                }
                Err(e) => eprintln!("{}", e),
            }

            let cpu_elapsed = now.elapsed() - total_elapsed;
            total_elapsed += cpu_elapsed;

            if cpu_elapsed < cpu_cycle_duration {
                let time_left = cpu_cycle_duration - cpu_elapsed;
                total_elapsed += time_left;
                std::thread::sleep(time_left);
            }

            if total_elapsed >= timer_cycle_duration {
                break 'cpu;
            }
        }

        Ok(())
    }
}
