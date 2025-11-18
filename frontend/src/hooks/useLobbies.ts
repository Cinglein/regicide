import { useState, useEffect, useCallback, useRef } from 'react';

const LOBBIES_ENDPOINT = '/lobbies';
const REFRESH_INTERVAL = 5000;

export function useLobbies(autoRefresh: boolean = true) {
  const [lobbies, setLobbies] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const intervalRef = useRef<NodeJS.Timeout | null>(null);

  const fetchLobbies = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const response = await fetch(LOBBIES_ENDPOINT);
      if (!response.ok) {
        throw new Error(`Failed to fetch lobbies: ${response.statusText}`);
      }
      const data: string[] = await response.json();
      setLobbies(data);
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Unknown error';
      setError(message);
      console.error('Error fetching lobbies:', err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchLobbies();

    if (autoRefresh) {
      intervalRef.current = setInterval(fetchLobbies, REFRESH_INTERVAL);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [fetchLobbies, autoRefresh]);

  const refresh = useCallback(() => {
    fetchLobbies();
  }, [fetchLobbies]);

  return { lobbies, isLoading, error, refresh };
}
