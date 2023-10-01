use state::State;

pub mod display;
pub mod operation;
pub mod state;

fn main() {
    let mut state = State::default();

    loop {
        let op = state.next_operation();
        println!("{:?}", op);
        state.update(op);
    }
}
