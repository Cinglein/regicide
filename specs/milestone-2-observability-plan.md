# Milestone 2: Observability Framework

## Current State Analysis

**What's Already There:**
- `tracing` dependency in workspace (v0.1.41)
- `tower-http::trace::TraceLayer` applied to HTTP router
- No tracing subscriber initialized
- No tracing macros used anywhere
- Silent error handling throughout (`if let Err(_err)` with no logging)

**Problems:**
1. No visibility into system behavior
2. Errors silently swallowed everywhere
3. No way to debug production issues
4. No performance metrics
5. No correlation between events (user actions → game state changes)

## Observability Goals

1. **Structured Logging**: Use tracing for all logging with structured fields
2. **Span Hierarchy**: Clear parent-child relationships between operations
3. **Error Visibility**: Log all error cases with context
4. **Performance Metrics**: Track key metrics (actor count, message rate, tick duration)
5. **Request Tracing**: Correlate user actions through the system
6. **Production Ready**: Configurable log levels, JSON output option

## Architecture Tracing Map

### Layer 1: HTTP/Server
- Server startup and shutdown
- HTTP request handling (static files, API endpoints)
- WebSocket upgrade requests

### Layer 2: WebSocket
- Connection establishment
- Join message handling
- Action message parsing
- Message send/receive
- Connection close/errors

### Layer 3: Actor System
- Actor system initialization
- Actor loop ticks (with timing)
- Join request processing
- Actor creation/destruction
- User connection/disconnection tracking
- Actor list updates

### Layer 4: Individual Actors
- Actor spawn
- Action processing
- State updates
- Message broadcasting

### Layer 5: Game Logic
- Game initialization
- Phase transitions (Play → Defend → Jester → Victory/Defeat)
- Card operations (play, discard, draw)
- Combo validation
- Damage calculation
- Win/loss conditions

## Implementation Plan

### Phase 1: Tracing Infrastructure Setup

**1.1 Add Dependencies**
```toml
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
tracing-appender = "0.2"
```

**1.2 Initialize Tracing Subscriber**
Create `backend/src/tracing.rs`:
- Configure subscriber with env filter (RUST_LOG)
- Support both human-readable (dev) and JSON (prod) formats
- Add file appender for persistent logs
- Include timestamps, thread IDs, and structured fields

**1.3 Configure Log Levels**
Environment variable structure:
```
RUST_LOG=regicide=debug,backend=debug,tower_http=info
```

### Phase 2: Server & HTTP Layer Tracing

**2.1 Server Startup (lib.rs)**
- Span: `server_start`
- Log: Actor system initialization
- Log: HTTP listener binding
- Log: Static file serving configuration
- Metrics: Server start time

**2.2 HTTP Layer (lib.rs)**
- TraceLayer already present, verify configuration
- Add request ID generation
- Log static file serves at TRACE level

**2.3 Lobby List Endpoint (list.rs)**
- Span: `get_lobby_list`
- Log: Number of lobbies returned
- Error handling: Log if actor list read fails

### Phase 3: WebSocket Layer Tracing

**3.1 Connection Lifecycle (ws.rs)**
- Span: `ws_connection` (includes connection ID)
- Event: Connection upgraded
- Event: Join message received (with user_id, lobby_id)
- Event: Action messages (with user_id, action type)
- Event: Connection closed
- Error: Parse errors, channel errors

**3.2 Message Handling**
- `read()`: Log each action received
  - Fields: user_id, action_type
  - Error: Deserialization failures
- `write()`: Log message broadcasts
  - Fields: user_id, message_type
  - Error: Send failures

### Phase 4: Actor System Tracing

**4.1 Actor Loop (actor.rs)**
- Span: `actor_loop` (runs entire lifetime)
- Metric: Tick duration (warn if > 10ms)
- Metric: Active actor count
- Metric: Active user count
- Debug: Messages processed per tick

**4.2 Join Request Processing**
- Span: `join_request` (user_id, actor_id)
- Event: New user connecting
- Event: Reconnecting user
- Event: New actor created
- Event: Joined existing actor
- Error: Channel send failures
- Error: User rejected (lobby full, game started)

**4.3 Disconnect Processing**
- Span: `disconnect` (user_id, actor_id)
- Event: User disconnected
- Event: Actor destroyed (last user left)
- Event: Actor list updated

**4.4 Actor Lifecycle**
- `Actor::spawn()`
  - Event: Actor created (user_id)
  - Fields: Initial state
- `Actor::update()`
  - Trace: Actions processed count
  - Debug: State changes

### Phase 5: Game Logic Tracing

**5.1 Action Processing (action.rs)**
- Span: `process_action` (user_id, actor_id, action_type)
- Each action variant gets detailed logging:

**Init:**
- Event: Game initialized
- Fields: player_count, hand_size, jesters_count, turn_order
- Error: Init when not in Uninit state

**Play:**
- Event: Cards played
- Fields: user_id, cards, combo_type, damage, total_damage
- Event: Phase transition (Play → Jester or Defend)
- Event: Enemy defeated
- Event: Victory condition
- Error: Invalid combo, not player's turn, no enemy

**Jester:**
- Event: Next player selected
- Fields: user_id, selected_player

**Discard:**
- Event: Cards discarded
- Fields: user_id, cards, defense_value, enemy_value
- Event: Defense successful/failed
- Event: Defeat condition
- Error: Insufficient defense

**5.2 State Transitions**
- Event: Phase change
  - Fields: from_phase, to_phase, current_player
- Event: Victory/Defeat
  - Fields: final_phase, remaining_enemies

### Phase 6: Deck & Card Operations

**6.1 Deck Operations (deck.rs)**
- Event: Card drawn (card_type)
- Event: Cards played to resolving
- Event: Cards healed back to library
- Event: Cards discarded
- Event: Next enemy engaged
- Metric: Library size, discard size, resolving count

**6.2 Combo Validation (card.rs)**
- Debug: Combo validation attempt
- Debug: Combo validated (type, strength, suit values)
- Error: Invalid combo rejected

### Phase 7: Error Handling Improvements

**Replace all silent error handling:**

Current pattern:
```rust
if let Err(_err) = send.send(msg) {}
```

New pattern:
```rust
if let Err(err) = send.send(msg) {
    error!(
        error = %err,
        message_type = "ServerMsg",
        "Failed to send message to user"
    );
}
```

**Error Categories:**
1. Channel send/receive failures → ERROR
2. Deserialization failures → WARN
3. WebSocket errors → INFO (client disconnect)
4. Invalid actions → DEBUG (validation failures)

### Phase 8: Performance Metrics

**Key Metrics to Track:**
1. **Actor System:**
   - Active actor count
   - Active user count
   - Messages processed per second
   - Average tick duration
   - Tick overruns (> 10ms)

2. **WebSocket:**
   - Active connections
   - Messages sent/received per second
   - Connection duration

3. **Game:**
   - Games in progress
   - Games completed (victory/defeat)
   - Average game duration
   - Actions per game

**Implementation:**
- Use tracing events with metrics fields
- Optional: Add `metrics` crate for prometheus export

### Phase 9: Span Context & Correlation

**Span Hierarchy:**
```
server
├─ actor_loop
│  ├─ join_request [user_id, actor_id]
│  │  └─ spawn_actor [user_id]
│  ├─ disconnect [user_id, actor_id]
│  └─ actor_update [actor_id]
│     └─ process_action [user_id, action]
│        ├─ validate_combo
│        ├─ apply_damage
│        └─ phase_transition
└─ ws_connection [connection_id]
   ├─ ws_read [user_id]
   └─ ws_write [user_id]
```

**Fields on All Spans:**
- `user_id`: String (when available)
- `actor_id`: Uuid (when available)
- `connection_id`: Uuid (for WS connections)

## Configuration

### Environment Variables

```bash
# Log level
RUST_LOG=regicide=debug,backend=debug,tower_http=info

# Log format (human or json)
LOG_FORMAT=human  # or json for production

# Log output
LOG_FILE=logs/regicide.log  # optional file output
```

### Log Levels by Module

```
ERROR:   Critical failures, system cannot continue
WARN:    Recoverable errors, degraded functionality
INFO:    Important state changes, user actions
DEBUG:   Detailed flow, useful for debugging
TRACE:   Very verbose, every message/event
```

**Recommended Defaults:**
- Development: `RUST_LOG=debug`
- Production: `RUST_LOG=info`
- Debugging: `RUST_LOG=trace`

## Testing Strategy

### Manual Testing
1. Start server with `RUST_LOG=debug`
2. Open browser, join lobby
3. Verify logs show:
   - Server startup
   - WS connection
   - Join request
   - Actor creation
   - Game init
4. Play game, verify action logging
5. Disconnect, verify cleanup logs

### Automated Testing
- Add `tracing-test` crate for testing
- Verify spans are created
- Verify error paths log errors
- Verify metrics are tracked

## Implementation Order

1. **Setup** (30 min)
   - Add dependencies
   - Create tracing.rs module
   - Initialize subscriber in main.rs

2. **Server Layer** (30 min)
   - Server startup logging
   - HTTP layer verification

3. **WebSocket Layer** (1 hour)
   - Connection lifecycle
   - Message read/write
   - Error handling

4. **Actor System** (1.5 hours)
   - Actor loop with metrics
   - Join/disconnect processing
   - Actor lifecycle

5. **Game Logic** (2 hours)
   - Action processing
   - State transitions
   - Phase changes
   - Error cases

6. **Deck Operations** (30 min)
   - Card operations
   - Combo validation

7. **Error Handling** (1 hour)
   - Replace all silent errors
   - Add context to all error logs

8. **Testing & Refinement** (1 hour)
   - Manual testing
   - Adjust log levels
   - Verify span hierarchy

**Total Estimated Time: 7.5 hours**

## Success Criteria

- [ ] All error cases logged with context
- [ ] Clear span hierarchy visible in logs
- [ ] Can trace a user action from WS → actor → game state
- [ ] Performance metrics visible (tick duration, message rate)
- [ ] Configurable via RUST_LOG env var
- [ ] JSON output option for production
- [ ] No silent error handling remaining
- [ ] Logs are useful for debugging production issues

## Example Log Output

**Human Format (Development):**
```
2025-11-18T12:00:00.123Z  INFO server_start: regicide::backend: Server starting addr=0.0.0.0:3000
2025-11-18T12:00:00.125Z  INFO actor_loop: regicide::backend::actor: Actor system initialized
2025-11-18T12:00:01.500Z  INFO ws_connection: regicide::backend::ws: WebSocket connection upgraded connection_id=a1b2c3d4
2025-11-18T12:00:01.510Z  INFO join_request: regicide::backend::actor: User joining user_id="alice" lobby=None
2025-11-18T12:00:01.512Z  INFO spawn_actor: regicide::backend::actor: New actor created actor_id=e5f6g7h8 user_id="alice"
2025-11-18T12:00:01.515Z  INFO process_action: regicide::backend::action: Game initialized action=Init players=2 hand_size=7
```

**JSON Format (Production):**
```json
{"timestamp":"2025-11-18T12:00:01.515Z","level":"INFO","target":"regicide::backend::action","span":{"name":"process_action","user_id":"alice","actor_id":"e5f6g7h8"},"fields":{"action":"Init","players":2,"hand_size":7},"message":"Game initialized"}
```

## Future Enhancements

1. **Metrics Export**: Add Prometheus endpoint
2. **Distributed Tracing**: Add OpenTelemetry support
3. **Log Aggregation**: Ship to ELK/Loki/DataDog
4. **Alerting**: Alert on error rates, slow ticks
5. **Dashboards**: Grafana dashboards for metrics
