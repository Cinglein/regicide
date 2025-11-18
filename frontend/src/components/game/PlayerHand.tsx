import type { JsCard } from '@/bindings/JsCard';
import { Card } from '@/components/shared/Card';

interface PlayerHandProps {
  hand: JsCard[];
  selectedIndices: number[];
  onCardClick: (index: number) => void;
  disabled?: boolean;
}

export function PlayerHand({ hand, selectedIndices, onCardClick, disabled = false }: PlayerHandProps) {
  return (
    <div className="bg-[#FAF9F6] dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 shadow-lg">
      <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100 mb-4">
        Your Hand
      </h2>
      {hand.length === 0 ? (
        <p className="text-center text-gray-500 dark:text-gray-500 py-8">
          No cards in hand
        </p>
      ) : (
        <div className="flex gap-2 flex-wrap justify-center">
          {hand.map((card, index) => (
            <Card
              key={index}
              card={card}
              selected={selectedIndices.includes(index)}
              disabled={disabled}
              onClick={() => onCardClick(index)}
            />
          ))}
        </div>
      )}
    </div>
  );
}
