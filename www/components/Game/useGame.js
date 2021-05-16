import * as wasm from 'wasm-mancala';
import { useState, useCallback, useRef, useEffect, useMemo } from 'react';

wasm.set_panic_hook();

const randomFirstMove = (gameConfig) => {
  return gameConfig.p1.player === 'AI' && gameConfig.p2.player === 'AI';
};

export function useGame(gameConfig) {
  const initGame = useMemo(
    () => wasm.GameJs.new(randomFirstMove(gameConfig)),
    []
  );

  const gameRef = useRef(initGame);
  const [board, setBoard] = useState(gameRef.current.board());

  const aiMove = useCallback(() => {
    const { current: game } = gameRef;

    const aiConfig =
      game.current_player() === 0 ? gameConfig.p1 : gameConfig.p2;
    const move = game.ai_move(
      aiConfig.algorithm,
      aiConfig.treeDepth,
      aiConfig.heuristics
    );

    console.log(game.current_player(), move);
    const turn_res = game.turn(move.hole);

    if (!turn_res) {
      throw new Error('AI made invalid move');
    }

    if (game.should_skip_next_move()) {
      game.skip_turn();
    }

    setTimeout(() => {
      setBoard(game.board());
    }, 1000);
  }, []);

  const onMove = useCallback(
    (move) => {
      const { current: game } = gameRef;

      if (
        (game.current_player() === 0 && gameConfig.p1.player === 'AI') ||
        (game.current_player() === 1 && gameConfig.p2.player === 'AI')
      ) {
        console.warn('Ignoring');
        return;
      }

      const turn_res = game.turn(move);
      if (!turn_res) {
        throw new Error('Invalid move');
      }

      if (game.should_skip_next_move()) {
        game.skip_turn();
      }

      setBoard(game.board());
    },
    [setBoard]
  );

  useEffect(() => {
    const { current: game } = gameRef;

    if (game.game_over()) return;

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

  const p1Turn = gameRef.current.current_player() === 0;

  return {
    game: gameRef.current,
    board,
    onP1Move: p1Turn ? onMove : null,
    onP2Move: !p1Turn ? onMove : null,
    onReplay: onReplay,
  };
}
