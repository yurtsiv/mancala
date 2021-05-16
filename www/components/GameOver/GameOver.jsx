import React from 'react';

export function GameOver({ winner, onReplay, goToMenu }) {
  return (
    <>
      <h1>{winner === 0 ? 'Player 1' : 'Player 2'} won</h1>
      <button onClick={onReplay}>Replay</button>
      <button onClick={goToMenu}>Menu</button>
    </>
  );
}
