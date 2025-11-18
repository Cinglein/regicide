import type { ClientMsg } from '@/bindings/ClientMsg';
import type { RegicideAction } from '@/bindings/RegicideAction';

export function createJoinMessage(clientToken: string, lobbyId: string | null): ClientMsg {
  return {
    Join: {
      lobby: lobbyId,
      client_token: clientToken,
    },
  };
}

export function createActionMessage(action: RegicideAction): ClientMsg {
  return {
    Action: {
      action,
    },
  };
}

export function sendWebSocketMessage(ws: WebSocket | null, msg: ClientMsg): boolean {
  if (!ws || ws.readyState !== WebSocket.OPEN) {
    return false;
  }
  try {
    ws.send(JSON.stringify(msg));
    return true;
  } catch (error) {
    console.error('Failed to send WebSocket message:', error);
    return false;
  }
}
