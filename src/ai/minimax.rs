use super::heuristics::*;
use super::types::*;
use crate::game::*;

pub fn minimax(game: &Game, depth: usize, maximizing_player: Player, ai_config: &AIConfig) -> AlgorithmRes {
  if depth == ai_config.tree_depth || game.game_over() {
    let eval = evaluate_game_state(&game, maximizing_player, &ai_config);
    return AlgorithmRes {
      best_move: None,
      eval,
      nodes_visited: 0
    }
  }

  if game.current_player() == maximizing_player {
    let mut max_eval = -f32::INFINITY;
    let mut max_eval_move = 0usize;
    let mut nodes_visited = 0usize;

    for player_move in 1..7 {
      let mut game_clone = game.clone();

      if game_clone.should_skip_next_move() {
        game_clone.skip_turn()
      } else if game_clone.turn(player_move) == None {
        // invalid move
        continue; 
      }

      let res = minimax(&game_clone, depth + 1, maximizing_player, ai_config);
      nodes_visited += res.nodes_visited + 1;

      if res.eval > max_eval {
        max_eval = res.eval;
        max_eval_move = player_move;
      };
    }

    return if depth == 0 {
      AlgorithmRes {
        best_move: Some(max_eval_move),
        eval: max_eval,
        nodes_visited
      }
    } else {
      AlgorithmRes {
        best_move: None,
        eval: max_eval,
        nodes_visited
      }
    };
  }

  let mut min_eval = f32::INFINITY;
  let mut nodes_visited = 0usize;

  for player_move in 1..7 {
    let mut game_clone = game.clone();

    if game_clone.should_skip_next_move() {
      game_clone.skip_turn()
    } else if game_clone.turn(player_move) == None {
      // invalid move
      continue; 
    }

    let res = minimax(&game_clone, depth + 1, maximizing_player, ai_config);
    nodes_visited += res.nodes_visited + 1;

    if res.eval < min_eval {
      min_eval = res.eval;
    };
  }

  return AlgorithmRes {
    best_move: None,
    eval: min_eval,
    nodes_visited
  }
}
