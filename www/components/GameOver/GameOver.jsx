import React from 'react';

export function GameOver({ game, onReplay, goToMenu }) {
  return (
    <>
      <h1>
        {game.winner() === 0
          ? 'Player 1 won'
          : game.winner() === 1
          ? 'Player 2 won'
          : 'Tie'}
      </h1>
      <h2>
        {game.p1_score()} - {game.p2_score()}
      </h2>
      <button onClick={onReplay}>Replay</button>
      <button onClick={goToMenu}>Menu</button>
    </>
  );
}
