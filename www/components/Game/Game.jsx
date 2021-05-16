import React from 'react';
import { GameBoard } from '../GameBoard';
import { GameOver } from '../GameOver';
import { useGame } from './useGame';

export function Game({ goToMenu }) {
  const { game, board, onReplay, onP1Move, onP2Move } = useGame();

  return (
    <>
      {game.game_over() ? (
        <GameOver winner={0} goToMenu={goToMenu} onReplay={onReplay} />
      ) : (
        <>
          <button onClick={onReplay}>Replay</button>
          <button onClick={goToMenu}>Menu</button>
          <GameBoard onP1Move={onP1Move} onP2Move={onP2Move} board={board} />
        </>
      )}
    </>
  );
}
