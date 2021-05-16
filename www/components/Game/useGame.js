import * as wasm from 'wasm-mancala';
import { useState, useCallback, useRef, useEffect, useMemo } from 'react';

const randomFirstMove = (gameConfig) => {
  console.log('calling');
  return gameConfig.p1.player === 'AI' && gameConfig.p2.player === 'AI';
};

export function useGame(gameConfig) {
  const initGame = useMemo(
    () => wasm.GameJs.new(randomFirstMove(gameConfig)),
    []
  );
  const gameRef = useRef(initGame);
  const { current: game } = gameRef;
  const [board, setBoard] = useState(game.board());

  const aiMove = useCallback(() => {
    const max = 7;
    const min = 1;
    const move = Math.random() * (max - min) + min;
    const turn_res = game.turn(move);
    if (!turn_res) {
      throw new Error('AI made invalid move');
    }

    setTimeout(() => {
      setBoard(game.board());
    }, 1000);
  }, []);

  const onMove = useCallback(
    (move) => {
      if (
        (game.current_player() === 0 && gameConfig.p1.player === 'AI') ||
        (game.current_player() === 1 && gameConfig.p2.player === 'AI')
      ) {
        console.warn('Ignoring');
        return;
      }

      const turn_res = game.turn(move);
      if (!turn_res) {
        throw new Error('AI made invalid move');
      }

      if (game.should_skip_next_move()) {
        game.skip_turn();
      }

      setBoard(game.board());
    },
    [setBoard]
  );

  useEffect(() => {
    if (
      (game.current_player() === 0 && gameConfig.p1.player === 'AI') ||
      (game.current_player() === 1 && gameConfig.p2.player === 'AI')
    ) {
      aiMove();
    }
  }, [board]);

  const onReplay = useCallback(() => {
    const newGame = wasm.GameJs.new(randomFirstMove(gameConfig));
    gameRef.current = newGame;
    setBoard(newGame.board());
  }, [setBoard, gameConfig]);

  const p1Turn = game.current_player() === 0;

  return {
    game,
    board,
    onP1Move: p1Turn ? onMove : null,
    onP2Move: !p1Turn ? onMove : null,
    onReplay: onReplay,
  };
}
