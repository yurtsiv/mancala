use wasm_bindgen::prelude::*;
use crate::game::*;
use crate::ai;
use crate::ai::types::*;

#[wasm_bindgen]
pub struct GameJs {
  game: Game
}

#[wasm_bindgen]
impl GameJs {
  pub fn new(random_first_move: bool) -> GameJs {
    GameJs {
      game: Game::new(random_first_move)
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

  pub fn ai_move(&self, algorithm: Algorithm, tree_depth: usize, heuristics: &JsValue) -> AITurnRes {
    ai::turn::next_turn(&self.game, &AIConfig {
      algorithm,
      tree_depth,
      heuristics: heuristics.into_serde().unwrap()
    })
  }
}