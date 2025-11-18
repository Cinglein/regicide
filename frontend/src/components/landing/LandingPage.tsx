import { useClient } from '@/contexts/ClientContext';
import { ClientTokenInput } from './ClientTokenInput';
import { LobbyBrowser } from './LobbyBrowser';
import { ValleyBackground } from '@/components/shared/ValleyBackground';

interface LandingPageProps {
  onJoinSuccess: () => void;
}

export function LandingPage({ onJoinSuccess }: LandingPageProps) {
  const { clientToken, lobbyId, setClientToken } = useClient();

  return (
    <div className="relative min-h-screen flex items-center justify-center p-4">
      <ValleyBackground />

      <div className="relative z-10 w-full max-w-2xl space-y-6">
        <div className="text-center mb-8">
          <h1 className="text-4xl sm:text-5xl font-bold text-gray-800 dark:text-gray-100 mb-2">
            REGICIDE
          </h1>
          <p className="text-gray-600 dark:text-gray-400">
            A cooperative card game
          </p>
        </div>

        <ClientTokenInput
          value={clientToken || ''}
          onChange={setClientToken}
          lobbyId={lobbyId}
        />

        <LobbyBrowser onJoinSuccess={onJoinSuccess} />
      </div>
    </div>
  );
}
