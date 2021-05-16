import React, { useState } from 'react';
import { ALGORITHM, HEURISTIC, PLAYER_ROLE } from './const';

import './menu.css';

const DEFAULT_AI_CONFIG = {
  player: 'AI',
  treeDepth: 3,
  algorithm: 'Minimax',
  heuristics: ['ScoreDiff'],
};

function PlayerConfig({ setConfig, config }) {
  const updateConfig = (c) => {
    if (c.player === 'AI') {
      setConfig(DEFAULT_AI_CONFIG);
    } else {
      setConfig({ ...config, ...c });
    }
  };

  return (
    <fieldset className="ai-config">
      <select
        name="type"
        onChange={(e) => updateConfig({ player: e.target.value })}
      >
        {Object.entries(PLAYER_ROLE).map(([key, label]) => (
          <option key={key} value={key}>
            {label}
          </option>
        ))}
      </select>
      {config?.player === 'AI' && (
        <>
          <div className="form-input">
            <label htmlFor="tree-depth">Tree depth</label>
            <input
              value={config.treeDepth}
              name="tree-depth"
              type="number"
              min="1"
              onChange={(e) => updateConfig({ treeDepth: +e.target.value })}
            />
          </div>
          <div className="form-input">
            <label htmlFor="algorithm">Algorithm</label>
            <select
              value={config.algorithm}
              name="algorithm"
              onChange={(e) => updateConfig({ algorithm: e.target.value })}
            >
              {Object.entries(ALGORITHM).map(([key, label]) => (
                <option key={key} value={key}>
                  {label}
                </option>
              ))}
            </select>
          </div>
          <div className="form-input">
            <label htmlFor="heuristics">Heuristics</label>
            <select
              value={config.heuristics}
              name="heuristics"
              onChange={(e) => {
                updateConfig({
                  heuristics: Array.from(
                    e.target.selectedOptions,
                    (o) => o.value
                  ),
                });
              }}
              multiple
            >
              {Object.entries(HEURISTIC).map(([key, label]) => (
                <option key={key} value={key}>
                  {label}
                </option>
              ))}
            </select>
          </div>
        </>
      )}
    </fieldset>
  );
}

export function Menu({ startGame }) {
  const [config, setConfig] = useState({ p1: 'AI', p2: 'human' });

  return (
    <div className="menu">
      <h1 className="title">Menu</h1>
      <div className="player-config">
        <h2 className="player-title">Player 1</h2>
        <PlayerConfig
          config={config.p1}
          setConfig={(c) => setConfig({ ...config, p1: c })}
        />
      </div>
      <div className="player-config">
        <h2>Player 2</h2>
        <PlayerConfig
          config={config.p2}
          setConfig={(c) => setConfig({ ...config, p2: c })}
        />
      </div>
      <button className="start-btn" onClick={() => startGame(config)}>
        Start
      </button>
    </div>
  );
}
