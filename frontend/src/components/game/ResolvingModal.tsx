import type { JsCard } from '@/bindings/JsCard';
import { Card } from '@/components/shared/Card';

interface ResolvingModalProps {
  resolving: JsCard[][];
  onClose: () => void;
}

export function ResolvingModal({ resolving, onClose }: ResolvingModalProps) {
  return (
    <div
      className="fixed inset-0 z-50 flex items-center justify-center bg-black/40"
      onClick={onClose}
    >
      <div
        className="
          bg-[#FAF9F6] dark:bg-gray-900
          border-2 border-gray-300 dark:border-gray-700
          shadow-2xl rounded-xl p-6
          max-w-4xl w-full mx-4 max-h-[80vh] overflow-y-auto
        "
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex justify-between items-center mb-6">
          <h2 className="text-xl font-semibold text-gray-800 dark:text-gray-100">
            Resolving Cards
          </h2>
          <button
            onClick={onClose}
            className="
              text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200
              text-2xl font-bold leading-none
              transition-colors
            "
          >
            ×
          </button>
        </div>

        <div className="space-y-6">
          {resolving.length === 0 ? (
            <p className="text-center text-gray-500 dark:text-gray-500 py-8">
              No cards being resolved
            </p>
          ) : (
            resolving.map((combo, index) => (
              <div key={index}>
                <h3 className="text-sm font-medium text-gray-600 dark:text-gray-400 mb-2">
                  Combo {index + 1}
                </h3>
                <div className="flex gap-2 flex-wrap">
                  {combo.map((card, cardIndex) => (
                    <Card key={cardIndex} card={card} />
                  ))}
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    </div>
  );
}
