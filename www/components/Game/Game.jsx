import React from 'react';
import { GameBoard } from '../GameBoard';
import { useGame } from './useGame';

export function Game() {
  const { game, board, onP1Move, onP2Move } = useGame();

  return (
    <>
      <GameBoard onP1Move={onP1Move} onP2Move={onP2Move} board={board} />
    </>
  );
}
