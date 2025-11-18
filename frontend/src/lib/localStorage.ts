const STORAGE_KEYS = {
  CLIENT_TOKEN: 'regicide_client_token',
  LOBBY_ID: 'regicide_lobby_id',
} as const;

export function getClientToken(): string | null {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem(STORAGE_KEYS.CLIENT_TOKEN);
}

export function setClientToken(token: string): void {
  if (typeof window === 'undefined') return;
  localStorage.setItem(STORAGE_KEYS.CLIENT_TOKEN, token);
}

export function getLobbyId(): string | null {
  if (typeof window === 'undefined') return null;
  return localStorage.getItem(STORAGE_KEYS.LOBBY_ID);
}

export function setLobbyId(lobbyId: string | null): void {
  if (typeof window === 'undefined') return;
  if (lobbyId === null) {
    localStorage.removeItem(STORAGE_KEYS.LOBBY_ID);
  } else {
    localStorage.setItem(STORAGE_KEYS.LOBBY_ID, lobbyId);
  }
}

export function clearStorage(): void {
  if (typeof window === 'undefined') return;
  localStorage.removeItem(STORAGE_KEYS.CLIENT_TOKEN);
  localStorage.removeItem(STORAGE_KEYS.LOBBY_ID);
}
