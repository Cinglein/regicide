'use client';

import { useState } from 'react';
import { useGame } from '@/contexts/GameContext';
import { useWebSocket } from '@/contexts/WebSocketContext';
import { useClient } from '@/contexts/ClientContext';
import { LandingPage } from '@/components/landing/LandingPage';
import { GameBoard } from '@/components/game/GameBoard';
import { ReconnectingOverlay } from '@/components/shared/ReconnectingOverlay';

export default function Home() {
  const { gameState, setGameState } = useGame();
  const { connectionState, retryAttempt, nextRetryIn, reconnect, disconnect } = useWebSocket();
  const { setLobbyId } = useClient();
  const [isJoining, setIsJoining] = useState(false);

  const handleJoinSuccess = () => {
    setIsJoining(true);
  };

  const handleCancelReconnect = () => {
    disconnect();
    setLobbyId(null);
    setGameState(null);
    setIsJoining(false);
  };

  const isInGame =
    gameState !== null && typeof gameState === 'object' && 'Game' in gameState;

  const isVictory = gameState === 'Victory';
  const isDefeat = gameState === 'Defeat';
  const isGameEnded = isVictory || isDefeat;

  const showReconnecting = connectionState === 'reconnecting';

  if (showReconnecting) {
    return (
      <ReconnectingOverlay
        attempt={retryAttempt}
        nextRetryIn={nextRetryIn}
        onReconnect={reconnect}
        onCancel={handleCancelReconnect}
      />
    );
  }

  if (isGameEnded) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <h1 className="text-6xl font-bold mb-4">
            {isVictory ? '🎉 Victory!' : '💀 Defeat'}
          </h1>
          <p className="text-xl text-gray-600 dark:text-gray-400 mb-8">
            {isVictory
              ? 'Congratulations! You defeated all the enemies!'
              : 'Better luck next time!'}
          </p>
          <button
            onClick={handleCancelReconnect}
            className="
              px-6 py-3 rounded-lg
              bg-emerald-400 hover:bg-emerald-500 dark:bg-emerald-300 dark:hover:bg-emerald-400
              text-gray-800 dark:text-gray-900
              font-semibold shadow-sm hover:shadow-md
              transition-all
            "
          >
            Return to Lobby
          </button>
        </div>
      </div>
    );
  }

  if (isInGame && 'Game' in gameState) {
    return <GameBoard gameState={gameState} />;
  }

  if (isJoining || connectionState === 'connecting') {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="text-4xl mb-4 animate-spin">⟳</div>
          <p className="text-xl text-gray-600 dark:text-gray-400">Joining game...</p>
        </div>
      </div>
    );
  }

  return <LandingPage onJoinSuccess={handleJoinSuccess} />;
}
