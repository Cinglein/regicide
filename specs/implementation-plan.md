# Milestone 1: Implementation Plan

## Overview

Build a single-page React app for Regicide with soft pastel valley aesthetic, supporting both landing page (lobby browser) and game screen.

## Phase 1: Foundation & Setup

### 1.1 Update Tailwind Configuration
- Add custom parchment colors to `tailwind.config.ts`
- Configure custom color palette
- Set up dark mode

### 1.2 Create Valley Background Component
- SVG-based low-poly valley landscape
- Separate light/dark mode versions
- Responsive (adapts to mobile/desktop)

### 1.3 Set Up Contexts
- `ClientContext`: Manage client_token and lobby_id
- `WebSocketContext`: WebSocket connection state and messaging
- `GameContext`: Game state from ServerMsg

### 1.4 Create Utility Libraries
- `lib/websocket.ts`: WebSocket utilities, message builders
- `lib/validation.ts`: Frontend validation for actions
- `lib/cardUtils.ts`: Card parsing, suit/rank extraction
- `lib/localStorage.ts`: Persist and retrieve client data

### 1.5 Create Custom Hooks
- `useLocalStorage.ts`: Generic localStorage hook
- `useLobbies.ts`: Fetch lobbies with 5s auto-refresh
- `useWebSocket.ts`: WebSocket connection with reconnection logic

## Phase 2: Shared Components

### 2.1 Card Component (`components/shared/Card.tsx`)
- Display card with suit symbol and rank
- Support selected state (ring border)
- Support hover effects
- Parchment background in light mode
- Props: `card: Card`, `selected?: boolean`, `onClick?: () => void`

### 2.2 CardBack Component (`components/shared/CardBack.tsx`)
- Display card back with number
- Decorative double border
- Optional click handler
- Props: `count: number`, `label?: string`, `onClick?: () => void`

### 2.3 ConnectionStatus Component (`components/shared/ConnectionStatus.tsx`)
- Small badge showing connection state
- States: connected, connecting, reconnecting, disconnected
- Show retry count when reconnecting
- Position: Fixed in corner

## Phase 3: Landing Page

### 3.1 ClientTokenInput Component (`components/landing/ClientTokenInput.tsx`)
- Text input with 32 char max validation
- Character counter
- Save to localStorage on change
- Display current lobby ID (read-only, or editable?)
- Props: `value: string`, `onChange: (value: string) => void`

### 3.2 LobbyBrowser Component (`components/landing/LobbyBrowser.tsx`)
- Table of lobbies with ID and player count
- Click row to highlight (toggle)
- Only one row can be highlighted at once
- Auto-refresh every 5s
- Manual refresh button
- "Join Game" / "New Game" button (text changes based on selection)
- Props: None (uses contexts)

### 3.3 LandingPage Component (`components/landing/LandingPage.tsx`)
- Container for ClientTokenInput and LobbyBrowser
- Valley background
- Handle join/create logic
- Transition to game screen on successful join

## Phase 4: Game Screen Components

### 4.1 EnemyDisplay Component (`components/game/EnemyDisplay.tsx`)
- Large card display for current enemy
- Damage bar showing damage / (enemy_value * 2)
- Elevated styling with shadow
- Props: `enemy: Card`, `damage: number`

### 4.2 GameStats Component (`components/game/GameStats.tsx`)
- Display Library, Discard, Resolving as card backs
- Resolving is clickable (opens modal)
- Horizontal layout
- Props: `librarySize: number`, `discardSize: number`, `resolvingSize: number`, `onResolvingClick: () => void`

### 4.3 PlayerList Component (`components/game/PlayerList.tsx`)
- Show all players in turn order
- Display player ID and hand count
- Highlight current turn with arrow/indicator
- Show "You" label for current player
- Clickable during Jester phase
- Responsive: vertical list on mobile, sidebar on desktop
- Props: `players: Array<[string, number]>`, `currentPlayer?: string`, `selfId: string`, `phase: Phase`, `onPlayerClick?: (playerId: string) => void`

### 4.4 PlayerHand Component (`components/game/PlayerHand.tsx`)
- Display player's hand as card grid
- Multi-select support (click to toggle)
- Clear selection after action
- Props: `hand: Card[]`, `selectedIndices: number[]`, `onCardClick: (index: number) => void`, `disabled?: boolean`

### 4.5 PhaseIndicator Component (`components/game/PhaseIndicator.tsx`)
- Show current phase and whose turn
- Color-coded by phase type
- Props: `phase: Phase`

### 4.6 ActionButtons Component (`components/game/ActionButtons.tsx`)
- Start Game, Play, Discard buttons
- Enable/disable based on phase, turn, and selection
- Frontend validation before sending action
- Props: `phase: Phase`, `isMyTurn: boolean`, `selectedCards: number[]`, `onAction: (action: RegicideAction) => void`

### 4.7 ResolvingModal Component (`components/game/ResolvingModal.tsx`)
- Modal overlay with opaque panel
- Display each combo (Card[]) as a row of cards
- Close on X click or outside click
- Props: `resolving: Card[][]`, `onClose: () => void`

### 4.8 GameBoard Component (`components/game/GameBoard.tsx`)
- Main game container
- Compose all game components
- Handle layout switching (mobile vs desktop)
- Valley background
- Props: `gameState: ServerMsg` (when type is "Game")

## Phase 5: Reconnection & Error Handling

### 5.1 ReconnectingOverlay Component (`components/shared/ReconnectingOverlay.tsx`)
- Full-screen overlay when reconnecting
- Show attempt count and retry countdown
- Manual "Reconnect Now" button
- "Cancel" button returns to landing
- Props: `attempt: number`, `nextRetryIn: number`, `onReconnect: () => void`, `onCancel: () => void`

### 5.2 WebSocket Reconnection Logic
- Exponential backoff: 1s, 2s, 4s, 8s, 16s, 32s (max)
- Max attempts: 10
- After max attempts, show error and return to landing
- Manual reconnect resets backoff

## Phase 6: Main App Integration

### 6.1 Update `app/page.tsx`
- Render LandingPage or GameBoard based on connection state
- Handle state transitions
- Manage WebSocket connection lifecycle

### 6.2 Update `app/layout.tsx`
- Wrap with context providers
- Add dark mode support
- Valley background (global or per-page?)

## Implementation Order

### Step 1: Foundation
1. Update `tailwind.config.ts` with custom colors
2. Create `ValleyBackground.tsx` component
3. Create contexts: ClientContext, WebSocketContext, GameContext
4. Create utility libraries: websocket, validation, cardUtils, localStorage
5. Create custom hooks: useLocalStorage, useLobbies, useWebSocket

### Step 2: Shared Components
1. `Card.tsx`
2. `CardBack.tsx`
3. `ConnectionStatus.tsx`

### Step 3: Landing Page
1. `ClientTokenInput.tsx`
2. `LobbyBrowser.tsx`
3. `LandingPage.tsx`

### Step 4: Game Components
1. `PhaseIndicator.tsx`
2. `EnemyDisplay.tsx`
3. `GameStats.tsx`
4. `ResolvingModal.tsx`
5. `PlayerHand.tsx`
6. `PlayerList.tsx`
7. `ActionButtons.tsx`
8. `GameBoard.tsx`

### Step 5: Reconnection
1. `ReconnectingOverlay.tsx`
2. Add reconnection logic to useWebSocket

### Step 6: Integration
1. Update `app/page.tsx` with state machine
2. Update `app/layout.tsx` with providers
3. Test full flow

## Detailed Component Specs

### Card Component

```typescript
interface CardProps {
  card: Card;
  selected?: boolean;
  disabled?: boolean;
  onClick?: () => void;
}

// Rendering:
// - Parse card string (e.g., "HeartAce") to suit and rank
// - Display suit symbol (♥ ♠ ♦ ♣) and rank
// - Suit colors: Hearts/Diamonds red, Spades/Clubs dark, Joker purple
// - Parchment bg in light mode, dark gray in dark mode
// - Selected: ring-2 ring-emerald-400
```

### CardBack Component

```typescript
interface CardBackProps {
  count: number;
  label?: string;
  onClick?: () => void;
}

// Rendering:
// - Card-sized box with decorative double border
// - Centered number (count)
// - Optional label below card
// - Clickable if onClick provided (hover effect)
```

### LobbyBrowser Component

```typescript
interface LobbyBrowserProps {
  // Uses contexts internally
}

// State:
// - selectedLobby: string | null
// - lobbies: ActorId[]
// - isRefreshing: boolean

// Behavior:
// - Fetch lobbies every 5s automatically
// - Manual refresh button
// - Click row to select/deselect
// - Join button: if selected → join, else → create new
// - On join success → transition to game screen
```

### PlayerList Component

```typescript
interface PlayerListProps {
  players: Array<[string, number]>; // [userId, handCount]
  currentPlayer?: string;
  selfId: string;
  phase: Phase;
  onPlayerClick?: (playerId: string) => void;
}

// Rendering:
// - Each player shows: ID (or "You"), card back with count
// - Current turn: highlighted with border and arrow
// - Jester phase: clickable players (except self?)
// - Mobile: vertical list, Desktop: sidebar
```

### ActionButtons Component

```typescript
interface ActionButtonsProps {
  phase: Phase;
  isMyTurn: boolean;
  selectedCards: number[];
  players: Array<[string, number]>;
  selfId: string;
  onAction: (action: RegicideAction) => void;
}

// Validation:
// - Start Game: Always enabled (anyone can start)
// - Play: Enabled if Play phase, my turn, valid combo selected
// - Discard: Enabled if Defend phase, my turn, at least 1 card selected
// - Jester action: Handled by PlayerList click
```

## Validation Logic

### Card Combo Validation (`lib/validation.ts`)

```typescript
function isValidCombo(cards: Card[]): boolean {
  // Valid combos:
  // - Single card
  // - Multiple cards of same rank (pair, triple, quad)
  // - Must be all same rank if multiple
  return cards.length > 0 && (
    cards.length === 1 ||
    cards.every(c => getRank(c) === getRank(cards[0]))
  );
}

function canPlayCombo(combo: Card[], phase: Phase): boolean {
  // Can only play during Play phase
  return phase.hasOwnProperty('Play') && isValidCombo(combo);
}

function canDiscard(cards: Card[], phase: Phase): boolean {
  // Can discard any cards during Defend phase
  return phase.hasOwnProperty('Defend') && cards.length > 0;
}
```

## WebSocket Flow

### Connection Flow

```typescript
// 1. User enters token, selects/creates lobby
// 2. Connect to /ws
const ws = new WebSocket('ws://localhost:3000/ws');

// 3. Send Join message
ws.send(JSON.stringify({
  Join: {
    lobby: selectedLobbyId || null,
    client_token: clientToken
  }
}));

// 4. Receive ServerMsg
ws.onmessage = (event) => {
  const msg: ServerMsg = JSON.parse(event.data);

  if ('Join' in msg) {
    // Joined successfully, update state
    setLobbyId(msg.Join.joined);
    setIsInGame(true);
  } else if ('Game' in msg) {
    // Game state update
    setGameState(msg.Game);
  } else if (msg === 'Victory' || msg === 'Defeat') {
    // Game ended
    setGameState(msg);
  }
};
```

### Reconnection Flow

```typescript
// On disconnect during game
ws.onclose = () => {
  if (isInGame) {
    // Enter reconnection state
    setConnectionState('reconnecting');

    // Exponential backoff
    let attempt = 0;
    const reconnect = () => {
      if (attempt >= 10) {
        // Give up, return to landing
        setConnectionState('disconnected');
        setIsInGame(false);
        return;
      }

      const delay = Math.min(1000 * Math.pow(2, attempt), 32000);
      setTimeout(() => {
        attempt++;
        // Try to reconnect with stored lobby_id and client_token
        connectWebSocket(clientToken, lobbyId);
      }, delay);
    };

    reconnect();
  }
};
```

## State Management

### ClientContext

```typescript
interface ClientContextValue {
  clientToken: string | null;
  lobbyId: string | null;
  setClientToken: (token: string) => void;
  setLobbyId: (id: string | null) => void;
}

// Persists to localStorage on change
```

### WebSocketContext

```typescript
interface WebSocketContextValue {
  ws: WebSocket | null;
  connectionState: 'disconnected' | 'connecting' | 'connected' | 'reconnecting';
  retryAttempt: number;
  sendMessage: (msg: ClientMsg) => void;
  connect: (token: string, lobbyId: string | null) => void;
  disconnect: () => void;
  reconnect: () => void;
}
```

### GameContext

```typescript
interface GameContextValue {
  gameState: ServerMsg | null;
  isInGame: boolean;
}

// Receives updates from WebSocket
```

## Filesystem Structure

```
frontend/src/
├── app/
│   ├── layout.tsx
│   ├── page.tsx
│   └── globals.css
├── bindings/
│   ├── Card.ts
│   ├── ClientMsg.ts
│   ├── Phase.ts
│   ├── RegicideAction.ts
│   ├── ServerMsg.ts
│   └── Suit.ts
├── components/
│   ├── landing/
│   │   ├── ClientTokenInput.tsx
│   │   ├── LobbyBrowser.tsx
│   │   └── LandingPage.tsx
│   ├── game/
│   │   ├── GameBoard.tsx
│   │   ├── EnemyDisplay.tsx
│   │   ├── GameStats.tsx
│   │   ├── PlayerList.tsx
│   │   ├── PlayerHand.tsx
│   │   ├── ResolvingModal.tsx
│   │   ├── PhaseIndicator.tsx
│   │   └── ActionButtons.tsx
│   └── shared/
│       ├── Card.tsx
│       ├── CardBack.tsx
│       ├── ConnectionStatus.tsx
│       ├── ReconnectingOverlay.tsx
│       └── ValleyBackground.tsx
├── hooks/
│   ├── useWebSocket.ts
│   ├── useLobbies.ts
│   └── useLocalStorage.ts
├── lib/
│   ├── websocket.ts
│   ├── validation.ts
│   ├── cardUtils.ts
│   └── localStorage.ts
└── contexts/
    ├── GameContext.tsx
    ├── WebSocketContext.tsx
    └── ClientContext.tsx
```

## Testing Checklist

### Landing Page
- [ ] Client token input validates 32 char max
- [ ] Client token persists to localStorage
- [ ] Lobbies auto-refresh every 5s
- [ ] Manual refresh works
- [ ] Lobby selection highlights row
- [ ] Re-clicking row unhighlights
- [ ] "Join Game" joins selected lobby
- [ ] "New Game" creates new lobby when none selected
- [ ] Transition to game screen on successful join

### Game Screen
- [ ] Enemy card displays correctly
- [ ] Damage bar shows correct progress
- [ ] Library/Discard/Resolving show correct counts
- [ ] Resolving modal opens on click
- [ ] Modal shows all combos correctly
- [ ] Modal closes on X or outside click
- [ ] Player list shows all players
- [ ] Current turn player is highlighted
- [ ] "You" label appears for current player
- [ ] Hand displays all cards
- [ ] Card selection works (multi-select)
- [ ] Selected cards show ring border
- [ ] Start Game button works
- [ ] Play button validates combo
- [ ] Discard button works
- [ ] Jester player selection works
- [ ] Action buttons enable/disable correctly

### Reconnection
- [ ] On disconnect, reconnection overlay appears
- [ ] Retry counter increments
- [ ] Countdown shows time to next retry
- [ ] Manual reconnect works
- [ ] Cancel returns to landing
- [ ] After 10 failed attempts, returns to landing
- [ ] On successful reconnect, game state restored

### Responsive
- [ ] Mobile: vertical stack layout
- [ ] Desktop: centered board with sidebar
- [ ] All components scale appropriately
- [ ] Touch targets are 44px+ on mobile
- [ ] Cards are readable on all screen sizes

## Notes

- All components should handle loading/error states
- WebSocket messages should be typed using bindings
- Validation should happen client-side before sending
- localStorage should be accessed through utility functions
- Valley background should be memoized (expensive SVG)
- Card parsing should be memoized (used frequently)
