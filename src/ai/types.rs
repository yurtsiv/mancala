use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Clone, PartialEq)]
pub enum Algorithm {
  Minimax,
  AlphaBeta
}

#[wasm_bindgen]
#[derive(Clone, PartialEq, Deserialize)]
pub enum Heuristic {
  ScoreDiff,
  CaptureOpps,
  TurnKeepingMoves,
  WinningMoves
}

#[derive(Clone)]
pub struct AIConfig {
  pub algorithm: Algorithm,
  pub heuristics: Vec<Heuristic>,
  pub tree_depth: usize
}

#[wasm_bindgen]
pub struct AITurnRes {
  pub hole: usize,
  pub thinking_time: usize
}