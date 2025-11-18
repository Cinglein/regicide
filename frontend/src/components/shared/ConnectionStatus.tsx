import type { ConnectionState } from '@/contexts/WebSocketContext';

interface ConnectionStatusProps {
  state: ConnectionState;
  retryAttempt?: number;
}

export function ConnectionStatus({ state, retryAttempt = 0 }: ConnectionStatusProps) {
  const getStatusColor = () => {
    switch (state) {
      case 'connected':
        return 'bg-emerald-500';
      case 'connecting':
        return 'bg-yellow-500 animate-pulse';
      case 'reconnecting':
        return 'bg-orange-500 animate-pulse';
      case 'disconnected':
        return 'bg-red-500';
    }
  };

  const getStatusText = () => {
    switch (state) {
      case 'connected':
        return 'Connected';
      case 'connecting':
        return 'Connecting...';
      case 'reconnecting':
        return `Reconnecting (${retryAttempt + 1}/10)...`;
      case 'disconnected':
        return 'Disconnected';
    }
  };

  return (
    <div className="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400">
      <div className={`w-2 h-2 rounded-full ${getStatusColor()}`} />
      <span>{getStatusText()}</span>
    </div>
  );
}
