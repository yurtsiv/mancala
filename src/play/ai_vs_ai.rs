use std::io;
use std::io::Write;
use crate::game::*;
use crate::ai;
use crate::ai::types::*;
use super::utils::*;

pub fn simulate(ai1_config: &AIConfig, ai2_config: &AIConfig) {
  let mut game = Game::new(true);

  while !game.game_over() {
    if game.should_skip_next_move() {
      game.skip_turn();
      continue;
    }

    print_board(&game.board());
    let player_str = get_player_str(game.current_player());
    print!("\n{} turn: ", player_str); 
    io::stdout().flush().unwrap();

    let ai_turn_res = ai::turn::next_turn(
      &game,
      if game.current_player() == Player::Player1 {
        ai1_config
      } else {
        ai2_config
      }
    );

    print!(" {}", ai_turn_res.hole);
    if game.turn(ai_turn_res.hole) == None {
      panic!("AI made invalid move");
    }

    print!("\nMETRIC_time_{} {}", player_str, ai_turn_res.thinking_time);
  }

  print_board(&game.board());
  print!("\nGame over");
  print!("\nWinner: {:?}\n", game.winner());
}