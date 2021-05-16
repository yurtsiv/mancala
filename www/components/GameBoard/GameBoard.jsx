import React from 'react';
import { range } from '../../lib/range';

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

// export class BoardComponent extends HTMLElement {
//   constructor() {
//     super();

//     let template = document.getElementById("board-template");
//     let templateContent = template.content;
//     this.attachShadow({ mode: "open" }).appendChild(
//       templateContent.cloneNode(true)
//     );

//     this.onP1Move = null;
//     this.pnP2Move = null;

//     this.p1Holes = [];
//     this.p2Holes = [];
//     this.p1Well = this.shadowRoot.getElementById("p1-well");
//     this.p2Well = this.shadowRoot.getElementById("p2-well");

//     const board = this.shadowRoot.querySelector(".holes");

//     for (let i = 0; i < 12; i++) {
//       const hole = document.createElement("div");
//       hole.className = "hole";
//       board.appendChild(hole);

//       if (i < 6) {
//         this.p1Holes.unshift(hole);
//         this._setupP1ClickListener(hole, 5 - i);
//       } else {
//         this.p2Holes.push(hole);
//         this._setupP2ClickListener(hole, i - 5);
//       }
//     }
//   }

//   drawBoard(gameBoard) {
//     gameBoard.p1_board.forEach((marbles, idx) => {
//       this.p1Holes[idx].innerText = marbles;
//     });

//     gameBoard.p2_board.forEach((marbles, idx) => {
//       this.p2Holes[idx].innerText = marbles;
//     });

//     this.p1Well.innerText = gameBoard.p1_well;
//     this.p2Well.innerText = gameBoard.p2_well;
//   }
// }

// customElements.define("board-component", BoardComponent);
