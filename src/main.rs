pub mod game;
pub mod ai;
pub mod play;

fn main() {
  // play::human_vs_ai::start_console_ui();
  play::ai_vs_ai::simulate();
}
