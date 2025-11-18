# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a web-based implementation of the Regicide card game, built with:
- **Backend**: Rust (Axum, WebSocket, actor-based architecture)
- **Frontend**: Next.js 15 with React 19 and TypeScript
- Game rules: https://www.regicidegame.com/site_files/33132/upload_files/RegicideRulesA4.pdf?dl=1

## Development Commands

### Backend (Rust)
```bash
# Build the backend
cargo build

# Run the server (serves both backend and frontend)
cargo run

# Build backend only
cargo build -p backend

# Run tests
cargo test

# Check without building
cargo check
```

### Frontend (Next.js)
```bash
cd frontend

# Install dependencies
npm install

# Development server (use this only for frontend development)
npm run dev

# Build static export
npm run build

# Lint
npm run lint
```

### Full Stack Development
The main `cargo run` command starts the Axum server on port 3000, which:
- Serves the static Next.js build from `frontend/out`
- Provides WebSocket endpoint at `/ws`
- Provides Swagger UI at `/swagger-ui`
- Serves actor list endpoint

## Architecture

### Backend: Actor-Based Game System

The backend uses a custom actor model for managing game state and WebSocket connections:

**Core Components:**
- `ActorSystem`: Manages all game instances (actors) and user connections
- `Actor`: Represents a single game instance with shared state and per-user state
- `Action` trait: Defines game actions (implemented by `RegicideAction`)

**WebSocket Flow:**
1. Client connects to `/ws` and sends a `Join` message with lobby ID and client token
2. `ActorSystem` either creates a new actor or joins an existing one
3. Client sends `Action` messages containing game moves
4. Server broadcasts `ServerMsg` updates to all players in the game

**Key Files:**
- `backend/src/actor.rs`: Actor system implementation (ActorSystem, Actor, UserHandle)
- `backend/src/ws.rs`: WebSocket handlers and message types
- `backend/src/action.rs`: Game action implementation (RegicideAction)
- `backend/src/state.rs`: Game state (SharedState, UserState, ServerMsg)
- `backend/src/card.rs`, `deck.rs`: Game logic for cards and deck management

**State Management:**
- `SharedState`: Game-wide state (deck, phase, turn order, damage)
- `UserState`: Per-player state (hand of cards)
- Game phases: Play, Defend, Jester, Victory, Defeat

### Frontend: Next.js with TypeScript

**Structure:**
- `frontend/src/app/`: Next.js app router pages
- `frontend/src/bindings/`: TypeScript type definitions auto-generated from Rust using ts-rs

**Type Safety:**
The backend uses `ts-rs` to export TypeScript definitions from Rust types. When modifying backend types with `#[ts(export)]`, run the backend to regenerate `frontend/src/bindings/`.

### Workspace Structure

This is a Cargo workspace with three crates:
- Root crate: Main entry point (`src/main.rs`) that calls `backend::serve()`
- `backend`: Game logic and server implementation
- `macros`: Procedural macros (currently minimal)

## Common Patterns

### Adding a New Game Action

1. Add variant to `RegicideAction` enum in `backend/src/action.rs`
2. Implement handling in `RegicideAction::update()`
3. Update `ClientMsg` or `ServerMsg` in `backend/src/ws.rs` or `state.rs` if needed
4. Add `#[ts(export)]` to ensure TypeScript bindings are generated
5. Run backend to regenerate TypeScript bindings

### Modifying Game State

All state mutations happen in `Action::update()`. The actor system is single-threaded (runs in a dedicated thread with a 10ms tick), so no locks are needed within actor logic.

### WebSocket Message Format

Messages are JSON-serialized:
- Client → Server: `ClientMsg<RegicideAction>` (Join or Action)
- Server → Client: `ServerMsg` (Join or State)

## Type Generation

The backend uses `ts-rs` to generate TypeScript types. Types marked with `#[ts(export, export_to = "../../frontend/src/bindings/")]` are automatically exported when the backend runs.

## Build Artifacts

- Rust build artifacts: `/target`, `/backend/target`
- Next.js artifacts: `/frontend/.next`, `/frontend/out`
- TypeScript bindings: `/frontend/src/bindings/` (git-tracked, auto-generated)

## Frontend Architecture

The frontend is built with Next.js 15, React 19, TypeScript, and Tailwind CSS.

**Key Design:**
- Soft pastel valley theme with low-poly SVG background
- Opaque parchment-colored UI components (no glass-morphism)
- Responsive design: mobile (vertical stack) and desktop (sidebar layout)
- Dark mode support via system preference

**State Management:**
- `ClientContext`: Manages client_token and lobby_id (persisted to localStorage)
- `WebSocketContext`: WebSocket connection with auto-reconnection (exponential backoff, max 10 attempts)
- `GameContext`: Current game state from ServerMsg

**Component Structure:**
- `components/landing/`: Landing page, lobby browser, client token input
- `components/game/`: Game board, player list, hand, enemy display, action buttons
- `components/shared/`: Reusable components (Card, CardBack, ValleyBackground, etc.)

**Validation:**
Valid card combos (from backend/src/card.rs):
- Single card
- 4 twos (Quad)
- 3 twos or threes (Triple)
- 2 twos/threes/fours/fives (Pair)
- Any non-joker card + ace (Companion)

**WebSocket Flow:**
1. Connect to `/ws`
2. Send `ClientMsg::Join` with lobby_id (null for new) and client_token
3. Receive `ServerMsg::Join` with joined lobby ID
4. Game starts when any player sends `RegicideAction::Init`
5. Players send actions via `ClientMsg::Action`
6. Server broadcasts `ServerMsg::Game` updates
7. On disconnect, auto-reconnect with exponential backoff

**Running Frontend Standalone:**
```bash
cd frontend
npm run dev  # Development server on port 3000
npm run build  # Build static export
```

**Full Stack:**
The main `cargo run` serves both backend and frontend. Frontend builds are served from `frontend/out`.

## Observability

**Current State:**
- `tracing` is a workspace dependency but not yet used
- `tower-http::trace::TraceLayer` is applied but no subscriber is initialized
- All errors are silently swallowed (`if let Err(_err)`)

**Planned (Milestone 2):**
- Structured logging with tracing throughout the codebase
- Span hierarchy: server → actor_system → actor → action → game_logic
- All error cases logged with context
- Performance metrics (tick duration, message rate, actor count)
- Configurable via `RUST_LOG` environment variable
- JSON output option for production

See `specs/milestone-2-observability-plan.md` for complete implementation plan.
