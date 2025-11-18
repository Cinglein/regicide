'use client';

import { createContext, useContext, useState, useEffect, type ReactNode } from 'react';
import { getClientToken, setClientToken as saveClientToken, getLobbyId, setLobbyId as saveLobbyId } from '@/lib/localStorage';

interface ClientContextValue {
  clientToken: string | null;
  lobbyId: string | null;
  setClientToken: (token: string) => void;
  setLobbyId: (id: string | null) => void;
}

const ClientContext = createContext<ClientContextValue | null>(null);

export function ClientProvider({ children }: { children: ReactNode }) {
  const [clientToken, setClientTokenState] = useState<string | null>(null);
  const [lobbyId, setLobbyIdState] = useState<string | null>(null);

  useEffect(() => {
    setClientTokenState(getClientToken());
    setLobbyIdState(getLobbyId());
  }, []);

  const setClientToken = (token: string) => {
    setClientTokenState(token);
    saveClientToken(token);
  };

  const setLobbyId = (id: string | null) => {
    setLobbyIdState(id);
    saveLobbyId(id);
  };

  return (
    <ClientContext.Provider value={{ clientToken, lobbyId, setClientToken, setLobbyId }}>
      {children}
    </ClientContext.Provider>
  );
}

export function useClient() {
  const context = useContext(ClientContext);
  if (!context) {
    throw new Error('useClient must be used within ClientProvider');
  }
  return context;
}
