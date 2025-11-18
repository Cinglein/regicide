# Milestone 1: UI/UX Design and Filesystem Layout

## Overview

Single-page React app for Regicide card game with clean, modern, cel-shaded aesthetic. Supports both mobile and desktop with responsive design.

## User Flow

### Landing Page Flow
1. User sees landing page with client token input and current lobby ID (if stored)
2. User enters/edits client token (max 32 chars, validated)
3. Lobby list auto-refreshes every 5s (manual refresh button available)
4. User clicks lobby row to highlight (click again to unhighlight)
5. User clicks "Join Game" button:
   - If lobby highlighted → joins that lobby
   - If no lobby highlighted → creates new lobby
6. WS connection established, Join message sent
7. On successful join, game screen appears

### Game Screen Flow
1. Players wait in lobby, see other players joining
2. Any player can click "Start Game" (sends Init action)
3. Game begins, players take turns based on phase
4. Players select cards and perform actions
5. Game ends with Victory or Defeat

### Reconnection Flow
1. If WS disconnects during game, enter reconnection state
2. Show "Reconnecting..." status with exponential backoff
3. On reconnect, send Join message with stored lobby_id and client_token
4. Resume game from current state

## UI/UX Design Options

### Color Palette Options

**Option A: Navy/Teal (Professional, Calm)**
- Background: `#0f172a` (slate-900)
- Surface: `#1e293b` (slate-800)
- Primary: `#14b8a6` (teal-500)
- Accent: `#06b6d4` (cyan-500)
- Text: `#f1f5f9` (slate-100)
- Muted: `#64748b` (slate-500)

**Option B: Slate/Emerald (Modern, Fresh)**
- Background: `#0f172a` (slate-900)
- Surface: `#1e293b` (slate-800)
- Primary: `#10b981` (emerald-500)
- Accent: `#34d399` (emerald-400)
- Text: `#f1f5f9` (slate-100)
- Muted: `#64748b` (slate-500)

**Option C: Charcoal/Cyan (Tech, Sleek)**
- Background: `#111827` (gray-900)
- Surface: `#1f2937` (gray-800)
- Primary: `#06b6d4` (cyan-500)
- Accent: `#22d3ee` (cyan-400)
- Text: `#f9fafb` (gray-50)
- Muted: `#6b7280` (gray-500)

**Option D: Dark Blue/Gold (Elegant, Premium)**
- Background: `#0c4a6e` (sky-900)
- Surface: `#075985` (sky-800)
- Primary: `#fbbf24` (amber-400)
- Accent: `#fcd34d` (amber-300)
- Text: `#f0f9ff` (sky-50)
- Muted: `#7dd3fc` (sky-300)

### Layout Options

**Option A: Vertical Stack (Mobile-First)**
```
┌─────────────────────────┐
│   Enemy Card + Damage   │
│  Library/Discard Stats  │
├─────────────────────────┤
│   Resolving Cards       │
├─────────────────────────┤
│   Player List           │
│   (Turn Order)          │
├─────────────────────────┤
│   Your Hand             │
├─────────────────────────┤
│   Action Buttons        │
└─────────────────────────┘
```
- Best for mobile, scales well to desktop
- Clear top-to-bottom flow
- Easy to scan

**Option B: Centered Board (Desktop-Optimized)**
```
┌────────┬──────────────┬────────┐
│Players │ Enemy Card   │Connect │
│(Left)  │ + Damage     │Status  │
│        ├──────────────┤        │
│        │ Resolving    │        │
│        │ Cards        │        │
│        ├──────────────┤        │
│        │Stats (L/D)   │        │
├────────┴──────────────┴────────┤
│        Your Hand                │
├─────────────────────────────────┤
│     Action Buttons + Phase      │
└─────────────────────────────────┘
```
- Efficient use of widescreen space
- Game board feels centered/focused
- Players visible on sides

**Option C: Split View (Hybrid)**
```
┌─────────────────┬────────────┐
│  Enemy + Damage │  Players   │
│  Stats (L/D)    │  (Right)   │
│  Resolving      │            │
│  Cards          │            │
├─────────────────┴────────────┤
│       Your Hand               │
├───────────────────────────────┤
│   Action Buttons + Phase      │
└───────────────────────────────┘
```
- Balanced layout
- Clear separation of game board and players
- Works on both mobile (stacks) and desktop

## Screen Layouts

### Landing Page
```
┌─────────────────────────────────────┐
│         REGICIDE                    │
├─────────────────────────────────────┤
│ Client Token: [_______________] (edit)
│ Current Lobby: abc-123 (or "None")  │
├─────────────────────────────────────┤
│ ┌─ Available Lobbies ─────────┐    │
│ │ Lobby ID    │ Players       │    │
│ │─────────────┼──────────────│    │
│ │ abc-123     │ 2/4          │    │ ← clickable rows
│ │ xyz-789     │ 1/4          │    │   (highlight on click)
│ └─────────────────────────────┘    │
│ [Refresh]  [Join Game/New Game]    │
└─────────────────────────────────────┘
```

### Game Screen (Option A - Vertical Stack)
```
┌─────────────────────────────────────┐
│ ┌─ Enemy ──────────────────────┐   │
│ │   ♠ KING                     │   │
│ │   Damage: 15/20              │   │
│ └──────────────────────────────┘   │
│ Library: 24  Discard: 8             │
├─────────────────────────────────────┤
│ ┌─ Resolving ──────────────────┐   │
│ │  [Card] [Card]               │   │
│ └──────────────────────────────┘   │
├─────────────────────────────────────┤
│ ┌─ Players ────────────────────┐   │
│ │ → Alice (You): 6 cards       │   │ ← → indicates current turn
│ │   Bob: 5 cards               │   │   clickable for Jester
│ │   Carol: 7 cards             │   │
│ └──────────────────────────────┘   │
├─────────────────────────────────────┤
│ ┌─ Your Hand ──────────────────┐   │
│ │ [Card] [Card] [Card] [Card]  │   │ ← click to select
│ └──────────────────────────────┘   │
├─────────────────────────────────────┤
│ Phase: Play (Your Turn)             │
│ [Start Game] [Play] [Discard]       │ ← buttons enabled/disabled
└─────────────────────────────────────┘   based on phase/turn
  Connection: ● Connected
```

## Component Specifications

### Landing Page Components

**ClientTokenInput**
- Text input, max 32 chars
- Shows character count
- Validates on change
- Stored in localStorage

**LobbyBrowser**
- Table of lobbies (ID, player count)
- Auto-refresh every 5s
- Manual refresh button
- Click row to highlight (toggle)
- "Join Game" button (text changes based on selection)

### Game Screen Components

**EnemyDisplay**
- Shows current enemy card
- Shows damage / (enemy value * 2)
- Highlighted when active

**GameStats**
- Library count
- Discard count
- Compact, single line or badge style

**ResolvingCards**
- Shows cards currently being played/resolved
- Empty state when no cards

**PlayerList**
- Shows all players in turn order
- Highlights current turn player with arrow/indicator
- Shows hand count for each player
- Clickable when Jester phase (for player selection)
- Shows "You" label for current player

**PlayerHand**
- Shows your cards
- Click to select/deselect (visual feedback)
- Multi-select enabled

**PhaseIndicator**
- Shows current phase (Play, Defend, Jester, Victory, Defeat)
- Shows whose turn it is
- Color-coded based on phase

**ActionButtons**
- Start Game (visible before game starts)
- Play (enabled during Play phase on your turn)
- Discard (enabled during Defend phase on your turn)
- Buttons disabled when not your turn or wrong phase

**ConnectionStatus**
- Shows connection state (Connected, Disconnecting, Reconnecting)
- Shows retry count during reconnection
- Small, unobtrusive (corner badge)

**Card Component**
- Displays card text (e.g., "♥ ACE", "♠ KING", "JOKER")
- Selected state (border, background change)
- Compact size, responsive

## Interaction Patterns

### Card Selection
1. Click card → visual feedback (border, background)
2. Click again → deselect
3. Multiple cards can be selected
4. Selection cleared after action

### Lobby Selection
1. Click lobby row → highlight row
2. Click same row → unhighlight
3. Only one row can be highlighted at a time

### Player Selection (Jester Phase)
1. Only enabled during Jester phase on your turn
2. Click player in PlayerList → visual feedback
3. Send Jester action with selected player

### Action Validation
Frontend validates before sending:
- **Play**: Must be your turn, Play phase, valid card combo
- **Discard**: Must be your turn, Defend phase, at least one card
- **Jester**: Must be your turn, Jester phase, valid player selected
- **Init**: Always allowed (backend validates)

## Responsive Design

### Mobile (< 768px)
- Vertical stack layout (Option A)
- Full-width components
- Touch-friendly button sizes (min 44px)
- Cards in scrollable horizontal row if needed

### Tablet (768px - 1024px)
- Option A or C layout
- Larger cards, more spacing
- Side-by-side where appropriate

### Desktop (> 1024px)
- Option B or C layout
- Maximum width container (e.g., 1400px)
- Optimal use of horizontal space

## Filesystem Structure

```
frontend/src/
├── app/
│   ├── layout.tsx           (root layout, fonts, metadata)
│   ├── page.tsx             (main page: landing or game screen)
│   └── globals.css          (global styles)
├── bindings/                (auto-generated TypeScript types)
│   ├── Card.ts
│   ├── ClientMsg.ts
│   ├── Phase.ts
│   ├── RegicideAction.ts
│   ├── ServerMsg.ts
│   └── Suit.ts
├── components/
│   ├── landing/
│   │   ├── LobbyBrowser.tsx      (lobby table, selection, join/create)
│   │   └── ClientTokenInput.tsx  (token input with validation)
│   ├── game/
│   │   ├── GameBoard.tsx         (main game container)
│   │   ├── EnemyDisplay.tsx      (enemy card, damage counter)
│   │   ├── GameStats.tsx         (library, discard counts)
│   │   ├── PlayerList.tsx        (players, turn order, clickable)
│   │   ├── PlayerHand.tsx        (your cards, multi-select)
│   │   ├── ResolvingCards.tsx    (cards being resolved)
│   │   ├── PhaseIndicator.tsx    (current phase, turn info)
│   │   └── ActionButtons.tsx     (Play, Discard, Start Game)
│   └── shared/
│       ├── Card.tsx              (reusable card component)
│       └── ConnectionStatus.tsx  (connection state indicator)
├── hooks/
│   ├── useWebSocket.ts      (WS connection, send/receive, reconnection)
│   ├── useLobbies.ts        (fetch lobbies with 5s interval)
│   └── useLocalStorage.ts   (persist client_token, lobby_id)
├── lib/
│   ├── websocket.ts         (WS utilities, message helpers)
│   ├── validation.ts        (validate card combos, actions)
│   └── cardUtils.ts         (parse cards, get suit/rank/value)
└── contexts/
    ├── GameContext.tsx      (game state from ServerMsg)
    ├── WebSocketContext.tsx (WS connection state)
    └── ClientContext.tsx    (client_token, lobby_id)
```

## State Management

### Contexts

**ClientContext**
```typescript
{
  clientToken: string | null
  lobbyId: string | null
  setClientToken: (token: string) => void
  setLobbyId: (id: string | null) => void
}
```

**WebSocketContext**
```typescript
{
  ws: WebSocket | null
  connectionState: 'disconnected' | 'connecting' | 'connected' | 'reconnecting'
  sendMessage: (msg: ClientMsg) => void
  connect: (token: string, lobbyId: string | null) => void
  disconnect: () => void
}
```

**GameContext**
```typescript
{
  gameState: ServerMsg | null
  isInGame: boolean
}
```

### localStorage Keys
- `regicide_client_token`: string
- `regicide_lobby_id`: string | null

## Next Steps

After approval of this design:
1. Choose color palette (A, B, C, or D)
2. Choose layout option (A, B, or C)
3. Confirm any additional design details
4. Proceed to implementation
