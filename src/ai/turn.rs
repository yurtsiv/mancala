use super::minimax::*;
use super::alphabeta::*;
use super::types::*;
use crate::game::*;

pub fn next_turn(game: &Game, ai_config: &AIConfig) -> AITurnRes {
  let start_time = instant::Instant::now();

  let res = if ai_config.algorithm == Algorithm::AlphaBeta {
    alphabeta(
      &game,
      0,
      -f32::INFINITY,
      f32::INFINITY,
      game.current_player(),
      ai_config
    )
  } else {
    minimax(
      &game,
      0,
      game.current_player(),
      ai_config
    )
  };

  AITurnRes {
    hole: res.best_move.unwrap(),
    thinking_time: start_time.elapsed().as_millis() as usize,
    nodes_visited: res.nodes_visited
  }
}