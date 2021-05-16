import * as wasm from 'wasm-mancala';
import { useState, useCallback, useRef } from 'react';

export function useGame() {
  const gameRef = useRef(wasm.GameJs.new());
  const { current: game } = gameRef;
  const [board, setBoard] = useState(game.board());

  const onMove = useCallback(
    (move) => {
      const turn_res = game.turn(move);
      if (!turn_res) {
        console.log('INVALID MOVE');
        return;
      }

      if (game.should_skip_next_move()) {
        game.skip_turn();
      }

      setBoard(game.board());
    },
    [setBoard]
  );

  const onReplay = useCallback(() => {
    const newGame = wasm.GameJs.new();
    gameRef.current = newGame;
    setBoard(newGame.board());
  }, [setBoard]);

  const p1Turn = game.current_player() === 0;

  return {
    game,
    board,
    onP1Move: p1Turn ? onMove : null,
    onP2Move: !p1Turn ? onMove : null,
    onReplay: onReplay,
  };
}
