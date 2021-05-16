import React, { useState } from 'react';
import { Game } from './components/Game';
import { Menu } from './components/Menu';

export function App() {
  const [gameConfig, setGameConfig] = useState(null);

  return gameConfig ? (
    <Game gameConfig={gameConfig} goToMenu={() => setGameConfig(null)} />
  ) : (
    <Menu startGame={setGameConfig} />
  );
}
