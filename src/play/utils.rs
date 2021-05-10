use crate::game::*;

pub fn get_player_str(player: Player) -> &'static str {
  if player == Player::Player1 {
    "Player 1"
  } else {
    "Player 2"
  }
}

pub fn print_board(board: &Board) {
  print!("\n\n  ");
  for s in board.p1_board.iter().rev() {
    print!(" {}", s);
  }
  print!("\n{}               {}", board.p1_well, board.p2_well);
  print!("\n  ");
  for s in board.p2_board.iter() {
    print!(" {}", s);
  }

  print!("\n");
}
