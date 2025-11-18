import { useState, useEffect } from 'react';

interface ClientTokenInputProps {
  value: string;
  onChange: (value: string) => void;
  lobbyId: string | null;
}

const MAX_TOKEN_LENGTH = 32;

export function ClientTokenInput({ value, onChange, lobbyId }: ClientTokenInputProps) {
  const [localValue, setLocalValue] = useState(value);

  useEffect(() => {
    setLocalValue(value);
  }, [value]);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value.slice(0, MAX_TOKEN_LENGTH);
    setLocalValue(newValue);
    onChange(newValue);
  };

  const charCount = localValue.length;
  const isValid = charCount > 0 && charCount <= MAX_TOKEN_LENGTH;

  return (
    <div className="bg-[#FAF9F6] dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-6 shadow-lg">
      <div className="space-y-4">
        <div>
          <label
            htmlFor="clientToken"
            className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
          >
            Client Token
          </label>
          <input
            id="clientToken"
            type="text"
            value={localValue}
            onChange={handleChange}
            maxLength={MAX_TOKEN_LENGTH}
            placeholder="Enter your client token"
            className="
              w-full px-4 py-2 rounded-lg
              bg-[#FFFBF0] dark:bg-gray-800
              border border-gray-200 dark:border-gray-700
              text-gray-800 dark:text-gray-100
              placeholder-gray-400 dark:placeholder-gray-500
              focus:outline-none focus:ring-2 focus:ring-emerald-400
              transition-all
            "
          />
          <div className="flex justify-between items-center mt-1">
            <span
              className={`text-xs ${
                isValid
                  ? 'text-gray-500 dark:text-gray-500'
                  : 'text-red-500 dark:text-red-400'
              }`}
            >
              {charCount} / {MAX_TOKEN_LENGTH} characters
            </span>
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            Current Lobby
          </label>
          <div
            className="
              px-4 py-2 rounded-lg
              bg-gray-100 dark:bg-gray-800
              border border-gray-200 dark:border-gray-700
              text-gray-600 dark:text-gray-400
              text-sm
            "
          >
            {lobbyId || 'None'}
          </div>
        </div>
      </div>
    </div>
  );
}
