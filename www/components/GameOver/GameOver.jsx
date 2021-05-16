import React from 'react';

export function GameOver({ onReplay }) {
  return (
    <>
      <h1>Game over</h1>
      <button onClick={onReplay}>Replay</button>
      <button onClick={onReplay}>Menu</button>
    </>
  );
}
