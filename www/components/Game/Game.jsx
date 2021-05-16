import React from 'react';
import { GameBoard } from '../GameBoard';
import { GameOver } from '../GameOver';
import { useGame } from './useGame';

export function Game({ gameConfig, goToMenu }) {
  const { game, board, onReplay, onP1Move, onP2Move } = useGame(gameConfig);

  return (
    <>
      {game.game_over() ? (
        <GameOver game={game} goToMenu={goToMenu} onReplay={onReplay} />
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
