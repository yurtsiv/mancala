import * as wasm from "wasm-mancala";
import { BoardComponent } from "./components/BoardComoponent";

const boardComp = new BoardComponent();

document.body.appendChild(boardComp);

const game = wasm.GameJs.new();

boardComp.drawBoard(game.board());

boardComp.onP1Move = (idx) => {
  if (game.current_player() !== 0) {
    return;
  }

  console.log("p1 move", idx);

  if (game.game_over()) {
    alert(`Game over. Winner ${game.winner()}`);
    return;
  }

  game.turn(idx);
  boardComp.drawBoard(game.board());
};

boardComp.onP2Move = (idx) => {
  if (game.current_player() !== 1) {
    return;
  }

  console.log("p2 move", idx);

  if (game.game_over()) {
    alert(`Game over. Winner ${game.winner()}`);
    return;
  }

  game.turn(idx);
  boardComp.drawBoard(game.board());
};
