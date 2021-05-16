use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Algorithm {
  Minimax,
  AlphaBeta
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Heuristic {
  ScoreDiff,
  CaptureOpps,
  TurnKeepingMoves,
  WinningMoves
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AIConfig {
  pub algorithm: Algorithm,
  pub heuristics: Vec<Heuristic>,
  pub treeDepth: usize
}