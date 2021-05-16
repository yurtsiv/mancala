import React from 'react';

export function Menu({ startGame }) {
  return (
    <>
      <h1>Menu</h1>
      <button onClick={startGame}>Start</button>
    </>
  );
}
