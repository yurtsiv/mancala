import React from 'react';

import './gameBoard.css';

export function GameBoard({ board, onP1Move, onP2Move }) {
  return (
    <div className="board">
      <div className="well">{board.p1_well}</div>
      <div className="holes">
        {board.p1_board.reverse().map((marbles, i) => (
          <div
            key={i}
            onClick={() => (onP1Move ? onP1Move(6 - i) : () => {})}
            className={`hole ${onP1Move && marbles > 0 && 'clickable'}`}
          >
            {marbles}
          </div>
        ))}
        {board.p2_board.map((marbles, i) => (
          <div
            key={i}
            onClick={() => (onP2Move ? onP2Move(i + 1) : () => {})}
            className={`hole ${onP2Move && marbles > 0 && 'clickable'}`}
          >
            {marbles}
          </div>
        ))}
      </div>
      <div className="well">{board.p2_well}</div>
    </div>
  );
}
