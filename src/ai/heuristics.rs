use crate::game::*;
use super::params::*;

fn score_diff(game: &Game, maximizing_player: Player) -> f32 {
  let res = (game.p1_score() as f32 - game.p2_score() as f32) * (if maximizing_player == Player::Player1 {
    1
  } else {
    -1
  }) as f32;

  return res;
}

fn capture_opportunities(game: &Game, maximizing_player: Player) -> f32 {
  let mut captures = 0;
  let prev_player = game.current_player().clone();

  for i in 1..7 {
    let mut game_clone = game.clone();

    match game_clone.turn(i) {
      Some(res) => {
        if res.captured {
          if prev_player == maximizing_player {
            captures -= 1;
          } else {
            captures += 1;
          }
        }
      },
      _ => {}
    }
  }

  return captures as f32;
}

fn winning_moves(game: &Game, maximizing_player: Player) -> f32 {
  let mut moves = 0;

  let prev_player = game.current_player().clone();

  for i in 1..7 {
    let mut game_clone = game.clone();

    match game_clone.turn(i) {
      Some(_) => {
        if game_clone.game_over() {
          if prev_player == maximizing_player {
            moves -= 1;
          } else {
            moves += 1;
          }
        }
      }
      _ => {}
    }
  }

  return moves as f32;
}

fn turn_keeping_moves(game: &Game, maximizing_player: Player) -> f32 {
  let mut moves = 0;

  let prev_player = game.current_player().clone();

  for i in 1..7 {
    let mut game_clone = game.clone();

    match game_clone.turn(i) {
      Some(_) => {
        if game_clone.current_player() == prev_player {
          if prev_player == maximizing_player {
            moves -= 1;
          } else {
            moves += 1;
          }
        }
      }
      _ => {}
    }
  }

  return moves as f32;
}

pub fn evaluate_game_state(game: &Game, maximizing_player: Player) -> f32 {
  score_diff(game, maximizing_player) +
  2_f32 * capture_opportunities(game, maximizing_player) +
  10_f32 * turn_keeping_moves(game, maximizing_player) +
  1000_f32 * winning_moves(game, maximizing_player)
}