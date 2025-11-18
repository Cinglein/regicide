interface ReconnectingOverlayProps {
  attempt: number;
  nextRetryIn: number;
  onReconnect: () => void;
  onCancel: () => void;
}

export function ReconnectingOverlay({
  attempt,
  nextRetryIn,
  onReconnect,
  onCancel,
}: ReconnectingOverlayProps) {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/40">
      <div
        className="
          bg-[#FAF9F6] dark:bg-gray-900
          border-2 border-gray-300 dark:border-gray-700
          shadow-2xl rounded-xl p-8
          max-w-md w-full mx-4
        "
      >
        <div className="flex flex-col items-center gap-6 text-center">
          <div className="text-4xl animate-spin">⟳</div>

          <div>
            <h2 className="text-xl font-semibold text-gray-800 dark:text-gray-100 mb-2">
              Reconnecting...
            </h2>
            <p className="text-gray-600 dark:text-gray-400">
              Attempt {attempt + 1} of 10
            </p>
            {nextRetryIn > 0 && (
              <p className="text-sm text-gray-500 dark:text-gray-500 mt-1">
                Retrying in {nextRetryIn}s...
              </p>
            )}
          </div>

          <div className="flex gap-3 w-full">
            <button
              onClick={onReconnect}
              className="
                flex-1 px-6 py-2.5 rounded-lg
                bg-emerald-400 hover:bg-emerald-500 dark:bg-emerald-300 dark:hover:bg-emerald-400
                text-gray-800 dark:text-gray-900
                font-medium shadow-sm hover:shadow-md
                transition-all
              "
            >
              Reconnect Now
            </button>
            <button
              onClick={onCancel}
              className="
                flex-1 px-6 py-2.5 rounded-lg
                bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600
                text-gray-700 dark:text-gray-200
                font-medium shadow-sm hover:shadow-md
                transition-all
              "
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
