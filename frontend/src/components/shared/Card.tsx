import type { JsCard } from '@/bindings/JsCard';
import { getSuitSymbol, getSuitColor } from '@/lib/cardUtils';

interface CardProps {
  card: JsCard;
  selected?: boolean;
  disabled?: boolean;
  onClick?: () => void;
}

export function Card({ card, selected = false, disabled = false, onClick }: CardProps) {
  const symbol = getSuitSymbol(card.suit);
  const colorClass = getSuitColor(card.suit);
  const isClickable = onClick && !disabled;

  return (
    <button
      onClick={onClick}
      disabled={disabled || !onClick}
      className={`
        relative flex flex-col items-center justify-between
        w-16 h-24 sm:w-20 sm:h-32 md:w-24 md:h-36
        rounded-lg border
        bg-[#FFFBF0] dark:bg-gray-800
        border-gray-200 dark:border-gray-700
        shadow-sm
        transition-all duration-200
        ${selected ? 'ring-2 ring-emerald-400 shadow-md' : ''}
        ${isClickable ? 'hover:shadow-md hover:-translate-y-0.5 cursor-pointer' : ''}
        ${disabled ? 'opacity-50 cursor-not-allowed' : ''}
      `}
    >
      <span className={`text-sm sm:text-base md:text-lg font-semibold mt-1 ${colorClass}`}>
        {symbol}
      </span>
      <span className={`text-xl sm:text-2xl md:text-3xl font-bold ${colorClass}`}>
        {card.rank}
      </span>
      <span className={`text-sm sm:text-base md:text-lg font-semibold mb-1 ${colorClass}`}>
        {symbol}
      </span>
    </button>
  );
}
