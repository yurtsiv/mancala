use std::io;
use std::io::Write;
use crate::game::*;
use crate::ai;
use crate::ai::params::*;
use super::utils::*;

pub fn simulate() {
  let mut game = Game::new(true);

  while !game.game_over() {
    if game.should_skip_next_move() {
      game.skip_turn();
      continue;
    }

    print_board(&game.board());
    print!("\n{} turn: ", get_player_str(game.current_player())); 
    io::stdout().flush().unwrap();

    let hole = ai::turn::next_turn(&game);
    print!(" {}", hole);
    if game.turn(hole) == None {
      panic!("AI made invalid move");
    }
  }

  print_board(&game.board());
  print!("\nGame over");
  print!("\nWinner: {}\n", get_player_str(game.winner().unwrap()));
}