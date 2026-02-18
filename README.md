# 🎮 Twitch OAuth — Rust / Axum

A production-ready Twitch OAuth 2.0 + OpenID Connect service written in Rust using **Axum**.

## Architecture

```
src/
├── main.rs              # Server bootstrap, router assembly
├── config.rs            # Env-var config loading
├── error.rs             # Typed errors → HTTP responses
├── models.rs            # Serde structs (tokens, users, session data)
├── state.rs             # Shared AppState (config + HTTP client)
├── routes/
│   ├── auth.rs          # /auth/twitch/login, /callback, /logout
│   ├── user.rs          # /api/me, /api/me/validate  (protected)
│   └── health.rs        # /health
└── services/
    └── twitch.rs        # Token exchange, user fetch, revoke, validate
```

## OAuth Flow

```
Browser                    Your Server                  Twitch
   │                            │                          │
   │  GET /auth/twitch/login    │                          │
   │───────────────────────────►│                          │
   │                            │ store csrf_state→session │
   │  302 → Twitch auth URL     │                          │
   │◄───────────────────────────│                          │
   │                            │                          │
   │  GET https://id.twitch.tv/oauth2/authorize?...        │
   │──────────────────────────────────────────────────────►│
   │            user logs in & approves                    │
   │◄──────────────────────────────────────────────────────│
   │                            │                          │
   │  GET /auth/twitch/callback?code=...&state=...         │
   │───────────────────────────►│                          │
   │                            │ validate csrf_state      │
   │                            │                          │
   │                            │  POST /oauth2/token      │
   │                            │─────────────────────────►│
   │                            │  { access_token, ... }   │
   │                            │◄─────────────────────────│
   │                            │                          │
   │                            │  GET /helix/users        │
   │                            │─────────────────────────►│
   │                            │  { data: [user] }        │
   │                            │◄─────────────────────────│
   │                            │ store user → session     │
   │  302 → /api/me             │                          │
   │◄───────────────────────────│                          │
```

## Quick Start

### 1. Register your app on Twitch

1. Go to [https://dev.twitch.tv/console](https://dev.twitch.tv/console)
2. Click **Register Your Application**
3. Set **OAuth Redirect URL** to `http://localhost:3000/auth/twitch/callback`
4. Copy your **Client ID** and **Client Secret**

### 2. Configure environment

```bash
cp .env .env.local    # or just edit .env
```

Fill in:
```env
TWITCH_CLIENT_ID=your_client_id_here
TWITCH_CLIENT_SECRET=your_client_secret_here
TWITCH_REDIRECT_URI=http://localhost:3000/auth/twitch/callback
SESSION_SECRET=a-very-long-random-string
```

### 3. Run

```bash
cargo run
# or for release build:
cargo run --release
```

### 4. Test

| Action | URL |
|--------|-----|
| Login  | [http://localhost:3000/auth/twitch/login](http://localhost:3000/auth/twitch/login) |
| Me     | [http://localhost:3000/api/me](http://localhost:3000/api/me) |
| Validate token | [http://localhost:3000/api/me/validate](http://localhost:3000/api/me/validate) |
| Logout | [http://localhost:3000/auth/twitch/logout](http://localhost:3000/auth/twitch/logout) |
| Health | [http://localhost:3000/health](http://localhost:3000/health) |

## Endpoints

### `GET /auth/twitch/login`
Generates a CSRF `state` token, saves it to the session, and redirects the user to Twitch.

### `GET /auth/twitch/callback`
- Validates the `state` parameter (CSRF check)
- Exchanges the `code` for tokens via server-to-server POST to Twitch
- Fetches the user profile from `/helix/users`
- Stores `SessionUser` in the session
- Redirects to `/api/me`

### `GET /auth/twitch/logout`
Revokes the Twitch token and destroys the session.

### `GET /api/me` *(protected)*
Returns the authenticated user's profile from the session. Returns `401` if not logged in.

### `GET /api/me/validate` *(protected)*
Calls `https://id.twitch.tv/oauth2/validate` to check if the stored token is still active.

### `GET /health`
Returns `{ "status": "ok" }`.

## Production Checklist

- [ ] Replace `MemoryStore` with a persistent store (Redis via `tower-sessions-redis-store`)
- [ ] Set `with_secure(true)` on the session layer (requires HTTPS)
- [ ] Use a strong, randomly-generated `SESSION_SECRET` (32+ chars)
- [ ] Store tokens encrypted, never in plaintext
- [ ] Add token refresh logic using `refresh_token`
- [ ] Put the service behind a reverse proxy (nginx / Caddy)
- [ ] Add rate limiting on the `/auth/` routes

## Dependencies

| Crate | Purpose |
|-------|---------|
| `axum` | Web framework |
| `tokio` | Async runtime |
| `tower-sessions` | Cookie-based sessions |
| `reqwest` | HTTP client for Twitch API calls |
| `serde` | JSON serialization |
| `rand` | CSRF state generation |
| `tracing` | Structured logging |
| `thiserror` | Typed error definitions |
