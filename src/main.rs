pub mod game;
pub mod game_js;
pub mod ai;
pub mod play;

use crate::ai::types::*;

fn main() {
  let ai1_config = AIConfig {
    algorithm: Algorithm::Minimax,
    heuristics: vec![
      Heuristic::ScoreDiff,
      // Heuristic::CaptureOpps,
      // Heuristic::TurnKeepingMoves,
      // Heuristic::WinningMoves
    ],
    tree_depth: 7
  };

  let ai2_config = AIConfig {
    algorithm: Algorithm::Minimax,
    heuristics: vec![
      Heuristic::ScoreDiff,
      // Heuristic::CaptureOpps,
      // Heuristic::TurnKeepingMoves,
      // Heuristic::WinningMoves
    ],
    tree_depth: 7
  };

  // play::human_vs_ai::start_console_ui();
  play::ai_vs_ai::simulate(&ai1_config, &ai2_config);
}
