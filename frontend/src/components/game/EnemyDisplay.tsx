import type { JsCard } from '@/bindings/JsCard';
import { getSuitSymbol, getSuitColor } from '@/lib/cardUtils';

interface EnemyDisplayProps {
  enemy: JsCard;
  damage: number;
}

export function EnemyDisplay({ enemy, damage }: EnemyDisplayProps) {
  const symbol = getSuitSymbol(enemy.suit);
  const colorClass = getSuitColor(enemy.suit);
  const maxDamage = enemy.value * 2;
  const damagePercent = maxDamage > 0 ? (damage / maxDamage) * 100 : 0;

  return (
    <div className="flex flex-col items-center gap-3">
      <div
        className="
          relative flex flex-col items-center justify-between
          w-32 h-48 sm:w-40 sm:h-60 md:w-48 md:h-72
          rounded-lg border-2
          bg-[#FFFBF0] dark:bg-gray-800
          border-gray-300 dark:border-gray-600
          shadow-2xl transform -translate-y-2
        "
      >
        <span className={`text-2xl sm:text-3xl md:text-4xl font-semibold mt-2 ${colorClass}`}>
          {symbol}
        </span>
        <span className={`text-5xl sm:text-6xl md:text-7xl font-bold ${colorClass}`}>
          {enemy.rank}
        </span>
        <span className={`text-2xl sm:text-3xl md:text-4xl font-semibold mb-2 ${colorClass}`}>
          {symbol}
        </span>
      </div>

      <div className="w-full max-w-xs">
        <div className="flex justify-between text-sm text-gray-600 dark:text-gray-400 mb-1">
          <span>Damage</span>
          <span className="font-semibold">
            {damage} / {maxDamage}
          </span>
        </div>
        <div className="w-full h-3 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
          <div
            className="h-full bg-emerald-500 transition-all duration-300"
            style={{ width: `${Math.min(damagePercent, 100)}%` }}
          />
        </div>
      </div>
    </div>
  );
}
