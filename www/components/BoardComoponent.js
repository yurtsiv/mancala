export class BoardComponent extends HTMLElement {
  constructor() {
    super();

    let template = document.getElementById("board-template");
    let templateContent = template.content;
    this.attachShadow({ mode: "open" }).appendChild(
      templateContent.cloneNode(true)
    );

    this.onP1Move = null;
    this.pnP2Move = null;

    this.p1Holes = [];
    this.p2Holes = [];
    this.p1Well = this.shadowRoot.getElementById("p1-well");
    this.p2Well = this.shadowRoot.getElementById("p2-well");

    const board = this.shadowRoot.querySelector(".holes");

    for (let i = 0; i < 12; i++) {
      const hole = document.createElement("div");
      hole.className = "hole";
      board.appendChild(hole);

      if (i < 6) {
        this.p1Holes.unshift(hole);
        this._setupP1ClickListener(hole, 5 - i);
      } else {
        this.p2Holes.push(hole);
        this._setupP2ClickListener(hole, i - 5);
      }
    }
  }

  _setupP1ClickListener(hole, idx) {
    hole.addEventListener("click", () => {
      this.onP1Move(idx);
    });
  }

  _setupP2ClickListener(hole, idx) {
    hole.addEventListener("click", () => {
      this.onP2Move(idx);
    });
  }

  drawBoard(gameBoard) {
    gameBoard.p1_board.forEach((marbles, idx) => {
      this.p1Holes[idx].innerText = marbles;
    });

    gameBoard.p2_board.forEach((marbles, idx) => {
      this.p2Holes[idx].innerText = marbles;
    });

    this.p1Well.innerText = gameBoard.p1_well;
    this.p2Well.innerText = gameBoard.p2_well;
  }
}

customElements.define("board-component", BoardComponent);
