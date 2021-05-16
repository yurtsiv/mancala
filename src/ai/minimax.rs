use super::heuristics::*;
use super::types::*;
use crate::game::*;

// (best move, evaluation)
type MinimaxRes = (Option<usize>, f32);

pub fn minimax(game: &Game, depth: usize, maximizing_player: Player, ai_config: &AIConfig) -> MinimaxRes {
  if depth == ai_config.tree_depth || game.game_over() {
    let e = evaluate_game_state(&game, maximizing_player, &ai_config);
    return (None, e);
  }

  if game.current_player() == maximizing_player {
    let mut max_eval = -f32::INFINITY;
    let mut max_eval_move = 0usize;

    for player_move in 1..7 {
      let mut game_clone = game.clone();

      if game_clone.should_skip_next_move() {
        game_clone.skip_turn()
      } else if game_clone.turn(player_move) == None {
        // invalid move
        continue; 
      }

      let (_, eval) = minimax(&game_clone, depth + 1, maximizing_player, ai_config);

      if eval > max_eval {
        max_eval = eval;
        max_eval_move = player_move;
      };
    }

    return if depth == 0 {
      (Some(max_eval_move), max_eval)
    } else {
      (None, max_eval)
    };
  }

  let mut min_eval = f32::INFINITY;

  for player_move in 1..7 {
    let mut game_clone = game.clone();

    if game_clone.should_skip_next_move() {
      game_clone.skip_turn()
    } else if game_clone.turn(player_move) == None {
      // invalid move
      continue; 
    }

    let (_, eval) = minimax(&game_clone, depth + 1, maximizing_player, ai_config);

    if eval < min_eval {
      min_eval = eval;
    };
  }

  return (None, min_eval);
}
