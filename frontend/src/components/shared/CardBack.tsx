interface CardBackProps {
  count: number;
  label?: string;
  onClick?: () => void;
}

export function CardBack({ count, label, onClick }: CardBackProps) {
  const isClickable = !!onClick;

  return (
    <div className="flex flex-col items-center gap-1">
      <button
        onClick={onClick}
        disabled={!onClick}
        className={`
          relative flex items-center justify-center
          w-16 h-24 sm:w-20 sm:h-32 md:w-24 md:h-36
          rounded-lg
          bg-[#FFFBF0] dark:bg-gray-800
          border-double border-4
          border-emerald-300 dark:border-emerald-400/50
          text-gray-700 dark:text-gray-100
          shadow-sm
          transition-all duration-200
          ${isClickable ? 'hover:shadow-md hover:scale-105 cursor-pointer' : 'cursor-default'}
        `}
      >
        <span className="text-3xl sm:text-4xl md:text-5xl font-bold">{count}</span>
      </button>
      {label && (
        <span className="text-xs sm:text-sm text-gray-600 dark:text-gray-400 font-medium">
          {label}
        </span>
      )}
    </div>
  );
}
