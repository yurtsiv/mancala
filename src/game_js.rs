use wasm_bindgen::prelude::*;
use crate::game::*;

#[wasm_bindgen]
pub struct GameJs {
  game: Game
}

#[wasm_bindgen]
impl GameJs {
  pub fn new() -> GameJs {
    GameJs {
      game: Game::new(false)
    }
  }

  pub fn skip_turn(&mut self) {
    self.game.skip_turn()
  }

  pub fn turn(&mut self, relative_hole: usize) -> Option<TurnResult> {
    self.game.turn(relative_hole)
  }

  pub fn game_over(&self) -> bool {
    self.game.game_over()
  }

  pub fn current_player(&self) -> Player {
    self.game.current_player()
  }

  pub fn winner(&self) -> Option<Player> {
    self.game.winner()
  }

  pub fn should_skip_next_move(&self) -> bool {
    self.game.should_skip_next_move()
  }

  pub fn board(&self) -> JsValue {
    JsValue::from_serde(&self.game.board()).unwrap()
  }

  pub fn p1_score(&self) -> usize {
    self.game.p1_score()
  }

  pub fn p2_score(&self) -> usize {
    self.game.p2_score()
  }
}