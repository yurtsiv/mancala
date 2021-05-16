use rand::Rng;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

const SEEDS_IN_HOLE: usize = 4;
const BOARD_LEN: usize = 14;
const PLAYER_1_WELL_IDX: usize = 6;
const PLAYER_2_WELL_IDX: usize = 13;

#[wasm_bindgen]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Player {
  Player1,
  Player2
}

#[derive(Serialize, Deserialize)]
pub struct Board {
  pub p1_board: Vec<usize>,
  pub p2_board: Vec<usize>,
  pub p1_well: usize,
  pub p2_well: usize
}

type BoardInternal = [usize; 14];

#[wasm_bindgen]
#[derive(PartialEq)]
pub struct TurnResult {
  pub captured: bool,
  pub next_player: Player
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Game {
  winner: Option<Player>,
  current_player: Player,
  game_over: bool,
  board: BoardInternal,
  should_skip_next_move: bool,
  random_first_move: bool,
  turns_count: usize,
  rng: rand::rngs::ThreadRng
}

impl Game {
  pub fn new(random_first_move: bool) -> Game {
    Game {
      current_player: Player::Player2,
      game_over: false,
      winner: None,
      should_skip_next_move: false,
      random_first_move,
      turns_count: 0,
      rng: rand::thread_rng(),
      board: [
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        0,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        SEEDS_IN_HOLE,
        0
      ]
    }
  }

  pub fn skip_turn(&mut self) {
      self.should_skip_next_move = false;
      self.switch_players();
  }

  pub fn turn(&mut self, relative_hole: usize) -> Option<TurnResult> {
    if self.game_over {
      return None
    }

    if self.should_skip_next_move {
      return None
    }

    let rel_hole = if self.random_first_move && self.turns_count == 0 {
      self.rng.gen_range(1, 7)
    } else {
      relative_hole
    };

    if rel_hole < 1 || rel_hole > 6 {
      return None
    };

    let hole = if self.current_player == Player::Player1 {
      rel_hole - 1
    } else {
      rel_hole + 6
    };

    if self.board[hole] == 0 {
      return None
    }

    let last_hole = self.distribute_seeds(hole);

    let captured = self.capture(last_hole);
    let finished_on_well =
      last_hole == PLAYER_1_WELL_IDX ||
      last_hole == PLAYER_2_WELL_IDX;

    self.should_skip_next_move = !captured && finished_on_well;
    self.switch_players();
    self.check_game_over();
    self.turns_count += 1;

    Some(TurnResult {
      captured,
      next_player: self.current_player
    })
  }

  pub fn game_over(&self) -> bool {
    self.game_over
  }

  pub fn current_player(&self) -> Player {
    self.current_player
  }

  pub fn winner(&self) -> Option<Player> {
    self.winner
  }

  pub fn should_skip_next_move(&self) -> bool {
    self.should_skip_next_move
  }

  pub fn board(&self) -> Board {
    Board {
      p1_board: self.board[..6].to_vec(),
      p2_board: self.board[7..13].to_vec(),
      p1_well: self.board[PLAYER_1_WELL_IDX],
      p2_well: self.board[PLAYER_2_WELL_IDX]
    }
  }

  pub fn p1_score(&self) -> usize {
    self.board[PLAYER_1_WELL_IDX]
  }

  pub fn p2_score(&self) -> usize {
    self.board[PLAYER_2_WELL_IDX]
  }

  pub fn clone(&self) -> Game {
    Game {
      current_player: self.current_player,
      game_over: self.game_over,
      winner: self.winner,
      should_skip_next_move: self.should_skip_next_move,
      board: self.board.clone(),
      random_first_move: self.random_first_move,
      turns_count: self.turns_count,
      rng: rand::thread_rng()
    }
  }

  fn switch_players(&mut self) {
    self.current_player = if self.current_player == Player::Player1 {
      Player::Player2
    } else {
      Player::Player1
    }
  }

  fn opposite_hole_idx(hole: usize) -> usize {
    (BOARD_LEN - 2) - hole
  }

  fn capture(&mut self, last_hole: usize) -> bool {
    if last_hole == PLAYER_1_WELL_IDX || last_hole == PLAYER_2_WELL_IDX {
      return false;
    }

    // hole was not empty
    if self.board[last_hole] - 1 != 0 {
      return false
    }

    let opposite_hole = Game::opposite_hole_idx(last_hole);

    // opposite hole is empty
    if self.board[Game::opposite_hole_idx(last_hole)] == 0 {
      return false
    }

    match (self.current_player, last_hole) {
      (Player::Player1, h) if h < 5 => {
        self.board[PLAYER_1_WELL_IDX] += self.board[opposite_hole] + self.board[last_hole];
        self.board[last_hole] = 0;
        self.board[opposite_hole] = 0;
        return true;
      }
      (Player::Player2, h) if h > 7 => {
        self.board[PLAYER_2_WELL_IDX] += self.board[opposite_hole] + self.board[last_hole];
        self.board[last_hole] = 0;
        self.board[opposite_hole] = 0;
        return true;
      }
      _ => {}
    }
  
    false
  }

  fn distribute_seeds(&mut self, hole: usize) -> usize {
    let mut seeds = self.board[hole];

    self.board[hole] = 0;

    let mut next_idx = hole + 1;
    while seeds != 0 {
      match (self.current_player, next_idx) {
        (Player::Player1, PLAYER_2_WELL_IDX) | (Player::Player2, PLAYER_1_WELL_IDX) => {},
        _ => {
          self.board[next_idx] += 1;
          seeds -= 1;
        }
      }

      if seeds != 0 {
        next_idx = (next_idx + 1) % BOARD_LEN
      }
    }

    next_idx
  }

  fn check_game_over(&mut self) {
    self.game_over =
      self.board[..6].iter().all(|&s| s == 0) ||
      self.board[7..13].iter().all(|&s| s == 0);
  
    
    if self.game_over {
      let mut player1_sum: usize = self.board[PLAYER_1_WELL_IDX];
      let mut player2_sum: usize = self.board[PLAYER_2_WELL_IDX];

      for i in 0..6 {
        player1_sum += self.board[i];
        self.board[i] = 0;
      }

      for i in 7..13 {
        player2_sum += self.board[i];
        self.board[i] = 0;
      }

      self.board[PLAYER_1_WELL_IDX] = player1_sum;
      self.board[PLAYER_2_WELL_IDX] = player2_sum;

      self.winner = if player1_sum > player2_sum {
        Some(Player::Player1)
      } else if player1_sum < player2_sum {
        Some(Player::Player2)
      } else {
        None
      }
    }
  }
}