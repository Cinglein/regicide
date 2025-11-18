import { useState } from 'react';
import { useLobbies } from '@/hooks/useLobbies';
import { useWebSocket } from '@/contexts/WebSocketContext';
import { useClient } from '@/contexts/ClientContext';

interface LobbyBrowserProps {
  onJoinSuccess: () => void;
}

export function LobbyBrowser({ onJoinSuccess }: LobbyBrowserProps) {
  const { lobbies, isLoading, error, refresh } = useLobbies(true);
  const { connect } = useWebSocket();
  const { clientToken, setLobbyId } = useClient();
  const [selectedLobby, setSelectedLobby] = useState<string | null>(null);

  const handleRowClick = (lobbyId: string) => {
    setSelectedLobby((prev) => (prev === lobbyId ? null : lobbyId));
  };

  const handleJoinGame = () => {
    if (!clientToken || clientToken.length === 0 || clientToken.length > 32) {
      alert('Please enter a valid client token (1-32 characters)');
      return;
    }

    const lobbyToJoin = selectedLobby;
    setLobbyId(lobbyToJoin);
    connect(clientToken, lobbyToJoin);
    onJoinSuccess();
  };

  const buttonText = selectedLobby ? 'Join Game' : 'New Game';

  return (
    <div className="bg-[#FAF9F6] dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-6 shadow-lg">
      <div className="flex items-center justify-between mb-4">
        <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-100">
          Available Lobbies
        </h2>
        <button
          onClick={refresh}
          disabled={isLoading}
          className="
            px-3 py-1.5 rounded-lg text-sm
            bg-gray-200 hover:bg-gray-300 dark:bg-gray-700 dark:hover:bg-gray-600
            text-gray-700 dark:text-gray-200
            disabled:opacity-50 disabled:cursor-not-allowed
            transition-all
          "
        >
          {isLoading ? 'Refreshing...' : 'Refresh'}
        </button>
      </div>

      {error && (
        <div className="mb-4 p-3 bg-red-100 dark:bg-red-900/20 border border-red-300 dark:border-red-800 rounded-lg">
          <p className="text-sm text-red-700 dark:text-red-400">{error}</p>
        </div>
      )}

      <div className="mb-4 border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-100 dark:bg-gray-800">
            <tr>
              <th className="px-4 py-2 text-left text-sm font-medium text-gray-700 dark:text-gray-300">
                Lobby ID
              </th>
              <th className="px-4 py-2 text-left text-sm font-medium text-gray-700 dark:text-gray-300">
                Players
              </th>
            </tr>
          </thead>
          <tbody>
            {lobbies.length === 0 ? (
              <tr>
                <td
                  colSpan={2}
                  className="px-4 py-8 text-center text-sm text-gray-500 dark:text-gray-500"
                >
                  {isLoading ? 'Loading lobbies...' : 'No lobbies available'}
                </td>
              </tr>
            ) : (
              lobbies.map((lobbyId) => (
                <tr
                  key={lobbyId}
                  onClick={() => handleRowClick(lobbyId)}
                  className={`
                    cursor-pointer transition-colors
                    ${
                      selectedLobby === lobbyId
                        ? 'bg-emerald-100 dark:bg-emerald-900/20'
                        : 'hover:bg-gray-50 dark:hover:bg-gray-800/50'
                    }
                  `}
                >
                  <td className="px-4 py-3 text-sm text-gray-800 dark:text-gray-200 font-mono">
                    {lobbyId}
                  </td>
                  <td className="px-4 py-3 text-sm text-gray-600 dark:text-gray-400">
                    ?
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      <button
        onClick={handleJoinGame}
        disabled={!clientToken || isLoading}
        className="
          w-full px-6 py-3 rounded-lg
          bg-emerald-400 hover:bg-emerald-500 dark:bg-emerald-300 dark:hover:bg-emerald-400
          text-gray-800 dark:text-gray-900
          font-semibold shadow-sm hover:shadow-md
          disabled:opacity-50 disabled:cursor-not-allowed
          transition-all
        "
      >
        {buttonText}
      </button>
    </div>
  );
}
