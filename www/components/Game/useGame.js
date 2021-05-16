import * as wasm from 'wasm-mancala';
import { useMemo, useState } from 'react';

export function useGame() {
  const game = useMemo(() => wasm.GameJs.new(), []);
  const [board, setBoard] = useState(game.board());

  function onMove(move) {
    const turn_res = game.turn(move);
    if (!turn_res) {
      alert('INVALID MOVE');
    }

    if (game.should_skip_next_move()) {
      game.skip_turn();
    }

    setBoard(game.board());
  }

  const p1Turn = game.current_player() === 0;

  return {
    game,
    board,
    onP1Move: p1Turn ? onMove : null,
    onP2Move: !p1Turn ? onMove : null,
  };
}
