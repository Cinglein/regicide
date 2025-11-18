'use client';

import { createContext, useContext, useState, type ReactNode } from 'react';
import type { ServerMsg } from '@/bindings/ServerMsg';

interface GameContextValue {
  gameState: ServerMsg | null;
  setGameState: (state: ServerMsg | null) => void;
  isInGame: boolean;
}

const GameContext = createContext<GameContextValue | null>(null);

export function GameProvider({ children }: { children: ReactNode }) {
  const [gameState, setGameState] = useState<ServerMsg | null>(null);

  const isInGame =
    gameState !== null &&
    (gameState === 'Victory' ||
      gameState === 'Defeat' ||
      (typeof gameState === 'object' && 'Game' in gameState));

  return (
    <GameContext.Provider value={{ gameState, setGameState, isInGame }}>
      {children}
    </GameContext.Provider>
  );
}

export function useGame() {
  const context = useContext(GameContext);
  if (!context) {
    throw new Error('useGame must be used within GameProvider');
  }
  return context;
}
