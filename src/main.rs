use std::fmt;
const ID: &str = "timer-wheel-910d2c";
#[derive(Debug)]
struct AppState { id: String, counter: u64 }
impl fmt::Display for AppState { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "AppState({}, count={})", self.id, self.counter) } }
impl AppState { fn new() -> Self { Self { id: ID.to_string(), counter: 0 } } fn increment(&mut self) { self.counter += 1; } }
fn main() { let mut state = AppState::new(); for _ in 0..5 { state.increment(); } println!("{}", state); }
