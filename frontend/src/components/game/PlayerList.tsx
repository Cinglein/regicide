import type { Phase } from '@/bindings/Phase';
import { CardBack } from '@/components/shared/CardBack';

interface PlayerListProps {
  players: Array<[string, number]>;
  currentPlayer?: string;
  selfId: string;
  phase: Phase;
  onPlayerClick?: (playerId: string) => void;
}

export function PlayerList({ players, currentPlayer, selfId, phase, onPlayerClick }: PlayerListProps) {
  const isJesterPhase = typeof phase === 'object' && 'Jester' in phase;
  const jesterPlayerId = isJesterPhase ? phase.Jester : null;
  const canSelectPlayers = isJesterPhase && jesterPlayerId === selfId;

  const handlePlayerClick = (playerId: string) => {
    if (canSelectPlayers && onPlayerClick) {
      onPlayerClick(playerId);
    }
  };

  return (
    <div className="bg-[#FAF9F6] dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 shadow-lg">
      <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-4">
        Players
      </h2>
      <div className="space-y-3">
        {players.map(([playerId, handCount]) => {
          const isCurrentPlayer = playerId === currentPlayer;
          const isSelf = playerId === selfId;
          const isClickable = canSelectPlayers && playerId !== selfId;

          return (
            <div
              key={playerId}
              onClick={() => handlePlayerClick(playerId)}
              className={`
                flex items-center gap-3 p-3 rounded-lg
                border-l-4 transition-all
                ${
                  isCurrentPlayer
                    ? 'border-l-emerald-500 bg-emerald-50 dark:bg-emerald-900/10'
                    : 'border-l-transparent'
                }
                ${isClickable ? 'hover:bg-gray-100 dark:hover:bg-gray-800 cursor-pointer' : ''}
              `}
            >
              {isCurrentPlayer && (
                <span className="text-emerald-500 font-bold">→</span>
              )}
              <div className="flex-1">
                <div className="font-medium text-gray-800 dark:text-gray-100">
                  {playerId} {isSelf && '(You)'}
                </div>
              </div>
              <div className="flex-shrink-0">
                <CardBack count={handCount} />
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
