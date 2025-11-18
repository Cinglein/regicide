import type { Phase } from '@/bindings/Phase';
import type { JsCard } from '@/bindings/JsCard';
import type { RegicideAction } from '@/bindings/RegicideAction';
import { canPlayCombo, canDiscard } from '@/lib/validation';

interface ActionButtonsProps {
  phase: Phase;
  selfId: string;
  selectedCards: JsCard[];
  selectedIndices: number[];
  onAction: (action: RegicideAction) => void;
  gameStarted: boolean;
}

export function ActionButtons({
  phase,
  selfId,
  selectedCards,
  selectedIndices,
  onAction,
  gameStarted,
}: ActionButtonsProps) {
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

  const isMyTurn = currentPlayer === selfId;

  const canPlay = canPlayCombo(selectedCards, phase, isMyTurn);
  const canDiscardNow = canDiscard(phase, isMyTurn);

  const handleStartGame = () => {
    onAction('Init');
  };

  const handlePlay = () => {
    if (canPlay) {
      onAction({
        Play: {
          cards: selectedIndices,
        },
      });
    }
  };

  const handleDiscard = () => {
    if (canDiscardNow) {
      onAction({
        Discard: {
          cards: selectedIndices,
        },
      });
    }
  };

  return (
    <div className="flex gap-3 flex-wrap justify-center">
      {!gameStarted && (
        <button
          onClick={handleStartGame}
          className="
            px-6 py-2.5 rounded-lg
            bg-emerald-400 hover:bg-emerald-500 dark:bg-emerald-300 dark:hover:bg-emerald-400
            text-gray-800 dark:text-gray-900
            font-semibold shadow-sm hover:shadow-md
            transition-all
          "
        >
          Start Game
        </button>
      )}

      {gameStarted && (
        <>
          <button
            onClick={handlePlay}
            disabled={!canPlay}
            className="
              px-6 py-2.5 rounded-lg
              bg-emerald-400 hover:bg-emerald-500 dark:bg-emerald-300 dark:hover:bg-emerald-400
              text-gray-800 dark:text-gray-900
              font-semibold shadow-sm hover:shadow-md
              disabled:bg-gray-200 dark:disabled:bg-gray-700
              disabled:text-gray-400 dark:disabled:text-gray-500
              disabled:cursor-not-allowed
              transition-all
            "
          >
            Play Cards
          </button>

          <button
            onClick={handleDiscard}
            disabled={!canDiscardNow}
            className="
              px-6 py-2.5 rounded-lg
              bg-orange-400 hover:bg-orange-500 dark:bg-orange-300 dark:hover:bg-orange-400
              text-gray-800 dark:text-gray-900
              font-semibold shadow-sm hover:shadow-md
              disabled:bg-gray-200 dark:disabled:bg-gray-700
              disabled:text-gray-400 dark:disabled:text-gray-500
              disabled:cursor-not-allowed
              transition-all
            "
          >
            Discard
          </button>
        </>
      )}
    </div>
  );
}
