'use client';

import { createContext, useContext, useState, useEffect, useRef, useCallback, type ReactNode } from 'react';
import type { ClientMsg } from '@/bindings/ClientMsg';
import type { ServerMsg } from '@/bindings/ServerMsg';
import { createJoinMessage, sendWebSocketMessage } from '@/lib/websocket';

export type ConnectionState = 'disconnected' | 'connecting' | 'connected' | 'reconnecting';

interface WebSocketContextValue {
  ws: WebSocket | null;
  connectionState: ConnectionState;
  retryAttempt: number;
  nextRetryIn: number;
  sendMessage: (msg: ClientMsg) => boolean;
  connect: (token: string, lobbyId: string | null) => void;
  disconnect: () => void;
  reconnect: () => void;
}

const WebSocketContext = createContext<WebSocketContextValue | null>(null);

const MAX_RETRY_ATTEMPTS = 10;
const BASE_DELAY = 1000;
const MAX_DELAY = 32000;

export function WebSocketProvider({
  children,
  onMessage,
}: {
  children: ReactNode;
  onMessage: (msg: ServerMsg) => void;
}) {
  const [ws, setWs] = useState<WebSocket | null>(null);
  const [connectionState, setConnectionState] = useState<ConnectionState>('disconnected');
  const [retryAttempt, setRetryAttempt] = useState(0);
  const [nextRetryIn, setNextRetryIn] = useState(0);

  const wsRef = useRef<WebSocket | null>(null);
  const retryTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const retryIntervalRef = useRef<NodeJS.Timeout | null>(null);
  const currentTokenRef = useRef<string | null>(null);
  const currentLobbyIdRef = useRef<string | null>(null);
  const shouldReconnectRef = useRef(false);

  const clearRetryTimers = useCallback(() => {
    if (retryTimeoutRef.current) {
      clearTimeout(retryTimeoutRef.current);
      retryTimeoutRef.current = null;
    }
    if (retryIntervalRef.current) {
      clearInterval(retryIntervalRef.current);
      retryIntervalRef.current = null;
    }
  }, []);

  const disconnect = useCallback(() => {
    shouldReconnectRef.current = false;
    clearRetryTimers();
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
    }
    setWs(null);
    setConnectionState('disconnected');
    setRetryAttempt(0);
    setNextRetryIn(0);
  }, [clearRetryTimers]);

  const connect = useCallback(
    (token: string, lobbyId: string | null) => {
      disconnect();

      currentTokenRef.current = token;
      currentLobbyIdRef.current = lobbyId;
      shouldReconnectRef.current = true;
      setConnectionState('connecting');
      setRetryAttempt(0);

      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const wsUrl = `${protocol}//${window.location.host}/ws`;
      const websocket = new WebSocket(wsUrl);

      websocket.onopen = () => {
        setConnectionState('connected');
        setRetryAttempt(0);
        setNextRetryIn(0);
        clearRetryTimers();

        const joinMsg = createJoinMessage(token, lobbyId);
        sendWebSocketMessage(websocket, joinMsg);
      };

      websocket.onmessage = (event) => {
        try {
          const msg: ServerMsg = JSON.parse(event.data);
          onMessage(msg);
        } catch (error) {
          console.error('Failed to parse WebSocket message:', error);
        }
      };

      websocket.onerror = (error) => {
        console.error('WebSocket error:', error);
      };

      websocket.onclose = () => {
        setWs(null);
        wsRef.current = null;

        if (shouldReconnectRef.current && retryAttempt < MAX_RETRY_ATTEMPTS) {
          setConnectionState('reconnecting');
          const delay = Math.min(BASE_DELAY * Math.pow(2, retryAttempt), MAX_DELAY);
          setNextRetryIn(Math.floor(delay / 1000));

          retryIntervalRef.current = setInterval(() => {
            setNextRetryIn((prev) => Math.max(0, prev - 1));
          }, 1000);

          retryTimeoutRef.current = setTimeout(() => {
            clearRetryTimers();
            setRetryAttempt((prev) => prev + 1);
            if (currentTokenRef.current) {
              connect(currentTokenRef.current, currentLobbyIdRef.current);
            }
          }, delay);
        } else if (shouldReconnectRef.current) {
          setConnectionState('disconnected');
          shouldReconnectRef.current = false;
        }
      };

      wsRef.current = websocket;
      setWs(websocket);
    },
    [disconnect, retryAttempt, onMessage, clearRetryTimers]
  );

  const reconnect = useCallback(() => {
    if (currentTokenRef.current) {
      clearRetryTimers();
      setRetryAttempt(0);
      connect(currentTokenRef.current, currentLobbyIdRef.current);
    }
  }, [connect, clearRetryTimers]);

  const sendMessage = useCallback(
    (msg: ClientMsg) => {
      return sendWebSocketMessage(ws, msg);
    },
    [ws]
  );

  useEffect(() => {
    return () => {
      shouldReconnectRef.current = false;
      clearRetryTimers();
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [clearRetryTimers]);

  return (
    <WebSocketContext.Provider
      value={{
        ws,
        connectionState,
        retryAttempt,
        nextRetryIn,
        sendMessage,
        connect,
        disconnect,
        reconnect,
      }}
    >
      {children}
    </WebSocketContext.Provider>
  );
}

export function useWebSocket() {
  const context = useContext(WebSocketContext);
  if (!context) {
    throw new Error('useWebSocket must be used within WebSocketProvider');
  }
  return context;
}
