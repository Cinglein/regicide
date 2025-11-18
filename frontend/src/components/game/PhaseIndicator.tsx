import type { Phase } from '@/bindings/Phase';

interface PhaseIndicatorProps {
  phase: Phase;
  selfId: string;
}

export function PhaseIndicator({ phase, selfId }: PhaseIndicatorProps) {
  const getPhaseText = (): string => {
    if (typeof phase === 'string') {
      return phase;
    }

    if ('Play' in phase) {
      const playerId = phase.Play;
      const isYourTurn = playerId === selfId;
      return isYourTurn ? 'Your Turn - Play' : `${playerId}'s Turn - Play`;
    }

    if ('Defend' in phase) {
      const playerId = phase.Defend;
      const isYourTurn = playerId === selfId;
      return isYourTurn ? 'Your Turn - Defend' : `${playerId}'s Turn - Defend`;
    }

    if ('Jester' in phase) {
      const playerId = phase.Jester;
      const isYourTurn = playerId === selfId;
      return isYourTurn ? 'Your Turn - Choose Next Player' : `${playerId} Choosing Next Player`;
    }

    return 'Unknown Phase';
  };

  const getPhaseColor = (): string => {
    if (typeof phase === 'string') {
      if (phase === 'Victory') {
        return 'bg-emerald-100 dark:bg-emerald-900/20 text-emerald-800 dark:text-emerald-300 border-emerald-300 dark:border-emerald-700';
      }
      if (phase === 'Defeat') {
        return 'bg-red-100 dark:bg-red-900/20 text-red-800 dark:text-red-300 border-red-300 dark:border-red-700';
      }
    }

    if ('Play' in phase) {
      return 'bg-blue-100 dark:bg-blue-900/20 text-blue-800 dark:text-blue-300 border-blue-300 dark:border-blue-700';
    }

    if ('Defend' in phase) {
      return 'bg-orange-100 dark:bg-orange-900/20 text-orange-800 dark:text-orange-300 border-orange-300 dark:border-orange-700';
    }

    if ('Jester' in phase) {
      return 'bg-purple-100 dark:bg-purple-900/20 text-purple-800 dark:text-purple-300 border-purple-300 dark:border-purple-700';
    }

    return 'bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-300 border-gray-300 dark:border-gray-700';
  };

  return (
    <div
      className={`
        inline-block px-4 py-2 rounded-lg border font-medium text-sm
        ${getPhaseColor()}
      `}
    >
      {getPhaseText()}
    </div>
  );
}
