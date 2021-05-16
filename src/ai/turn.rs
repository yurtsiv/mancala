use super::minimax::*;
use super::alphabeta::*;
use super::types::*;
use crate::game::*;

pub fn next_turn(game: &Game, ai_config: &AIConfig) -> usize {
  if ai_config.algorithm == Algorithm::AlphaBeta {
    alphabeta(
      &game,
      0,
      -f32::INFINITY,
      f32::INFINITY,
      game.current_player(),
      ai_config
    ).0.unwrap()
  } else {
    minimax(
      &game,
      0,
      game.current_player(),
      ai_config
    ).0.unwrap()
  }
}