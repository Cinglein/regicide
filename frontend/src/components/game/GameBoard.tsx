import { useState } from 'react';
import type { ServerMsg } from '@/bindings/ServerMsg';
import type { RegicideAction } from '@/bindings/RegicideAction';
import { ValleyBackground } from '@/components/shared/ValleyBackground';
import { ConnectionStatus } from '@/components/shared/ConnectionStatus';
import { EnemyDisplay } from './EnemyDisplay';
import { GameStats } from './GameStats';
import { ResolvingModal } from './ResolvingModal';
import { PlayerList } from './PlayerList';
import { PlayerHand } from './PlayerHand';
import { PhaseIndicator } from './PhaseIndicator';
import { ActionButtons } from './ActionButtons';
import { useWebSocket } from '@/contexts/WebSocketContext';
import { useClient } from '@/contexts/ClientContext';
import { createActionMessage } from '@/lib/websocket';

interface GameBoardProps {
  gameState: Extract<ServerMsg, { Game: unknown }>;
}

export function GameBoard({ gameState }: GameBoardProps) {
  const { sendMessage, connectionState, retryAttempt } = useWebSocket();
  const { clientToken } = useClient();
  const [selectedIndices, setSelectedIndices] = useState<number[]>([]);
  const [showResolvingModal, setShowResolvingModal] = useState(false);

  const { phase, players, library_size, discard_size, damage, enemy, hand, resolving } =
    gameState.Game;

  const gameStarted = typeof phase !== 'string' || phase === 'Victory' || phase === 'Defeat';

  const handleCardClick = (index: number) => {
    setSelectedIndices((prev) =>
      prev.includes(index) ? prev.filter((i) => i !== index) : [...prev, index]
    );
  };

  const handleAction = (action: RegicideAction) => {
    const msg = createActionMessage(action);
    const success = sendMessage(msg);
    if (success) {
      setSelectedIndices([]);
    }
  };

  const handlePlayerClick = (playerId: string) => {
    const action: RegicideAction = {
      Jester: {
        player: playerId,
      },
    };
    handleAction(action);
  };

  const selectedCards = selectedIndices.map((i) => hand[i]);
  const resolvingSize = resolving.reduce((sum, combo) => sum + combo.length, 0);

  const currentPlayer =
    typeof phase === 'object'
      ? 'Play' in phase
        ? phase.Play
        : 'Defend' in phase
        ? phase.Defend
        : 'Jester' in phase
        ? phase.Jester
        : null
      : null;

  return (
    <div className="relative min-h-screen">
      <ValleyBackground />

      <div className="relative z-10 min-h-screen flex flex-col md:grid md:grid-cols-[300px_1fr] lg:grid-cols-[350px_1fr] gap-4 p-4">
        <div className="hidden md:flex md:flex-col gap-4">
          <div className="flex items-center justify-between md:justify-start gap-3">
            <ConnectionStatus state={connectionState} retryAttempt={retryAttempt} />
          </div>
          <PlayerList
            players={players}
            currentPlayer={currentPlayer || undefined}
            selfId={clientToken || ''}
            phase={phase}
            onPlayerClick={handlePlayerClick}
          />
        </div>

        <div className="flex flex-col gap-4 max-w-6xl mx-auto w-full">
          <div className="md:hidden">
            <ConnectionStatus state={connectionState} retryAttempt={retryAttempt} />
          </div>

          <div className="flex flex-col items-center gap-4">
            <EnemyDisplay enemy={enemy} damage={damage} />
            <GameStats
              librarySize={library_size}
              discardSize={discard_size}
              resolvingSize={resolvingSize}
              onResolvingClick={() => setShowResolvingModal(true)}
            />
          </div>

          <div className="md:hidden">
            <PlayerList
              players={players}
              currentPlayer={currentPlayer || undefined}
              selfId={clientToken || ''}
              phase={phase}
              onPlayerClick={handlePlayerClick}
            />
          </div>

          <PlayerHand
            hand={hand}
            selectedIndices={selectedIndices}
            onCardClick={handleCardClick}
          />

          <div className="flex flex-col items-center gap-3">
            <PhaseIndicator phase={phase} selfId={clientToken || ''} />
            <ActionButtons
              phase={phase}
              selfId={clientToken || ''}
              selectedCards={selectedCards}
              selectedIndices={selectedIndices}
              onAction={handleAction}
              gameStarted={gameStarted}
            />
          </div>
        </div>
      </div>

      {showResolvingModal && (
        <ResolvingModal resolving={resolving} onClose={() => setShowResolvingModal(false)} />
      )}
    </div>
  );
}
