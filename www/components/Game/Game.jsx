import React from 'react';
import { GameBoard } from '../GameBoard';
import { GameOver } from '../GameOver';
import { useGame } from './useGame';

export function Game({ goToMenu }) {
  const { game, board, onReplay, onP1Move, onP2Move } = useGame();

  return (
    <>
      <button onClick={onReplay}>Replay</button>
      <button onClick={goToMenu}>Menu</button>
      {game.game_over() ? (
        <GameOver goToMenu={goToMenu} onReplay={onReplay} />
      ) : (
        <GameBoard onP1Move={onP1Move} onP2Move={onP2Move} board={board} />
      )}
    </>
  );
}
