use std::io;
use std::io::Write;
use crate::game::*;
use crate::ai;
use crate::ai::types::*;
use super::utils::*;

const AI_PLAYER: Player = Player::Player1;

fn clear_screen() {
  print!("{}[2J", 27 as char);
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn get_player_turn() -> Option<usize> {
  let mut input_text = String::new();
  io::stdin()
      .read_line(&mut input_text)
      .expect("failed to read from stdin");

  let trimmed = input_text.trim();
  match trimmed.parse::<usize>() {
      Ok(hole) => Some(hole),
      Err(_) => None
  }
}

pub fn start_console_ui(ai_config: &AIConfig) {
  let mut game = Game::new(false);

  // clear_screen();
  while !game.game_over() {
    if game.should_skip_next_move() {
      game.skip_turn();
      continue;
    }

    print_board(&game.board());
    print!("\n{} turn: ", get_player_str(game.current_player())); 
    io::stdout().flush().unwrap();

    if game.current_player() == AI_PLAYER {
      let hole = ai::turn::next_turn(&game, ai_config);
      print!(" {}", hole);
      if game.turn(hole) == None {
        panic!("AI made invalid move");
      }
    } else {
      match get_player_turn() {
        Some(hole) => {
          if game.turn(hole) == None {
            print!("\nInvalid move");
          }
        },
        _ => {}
      }
    }
  }

  print_board(&game.board());
  print!("\nGame over");
  print!("\nWinner: {}\n", get_player_str(game.winner().unwrap()));
}