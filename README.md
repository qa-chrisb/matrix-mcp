# matrix-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server for the
[Matrix](https://matrix.org) chat protocol, written in Rust on top of the two
official SDKs:

- [`rmcp`](https://crates.io/crates/rmcp) — the official MCP Rust SDK (server +
  stdio transport + `#[tool]` macros)
- [`matrix-sdk`](https://crates.io/crates/matrix-sdk) — the official Matrix Rust
  SDK

It lets an MCP client (Claude, or any other MCP-capable assistant) log in to a
Matrix homeserver, list rooms, read and send messages, and join rooms.

## Tools

| Tool            | Description |
|-----------------|-------------|
| `login`         | Log in with username/password. The session is persisted and reused on the next start. |
| `whoami`        | Report the current login state (user id, device, homeserver, joined-room count). |
| `sync`          | Run a single sync to refresh the local room list and state. |
| `list_rooms`    | List joined rooms with id, name, topic, and encryption state. |
| `send_message`  | Send a text message to a room (plain text or Markdown). |
| `read_messages` | Read the most recent messages from a room, in chronological order. |
| `join_room`     | Join a room by id (`!room:server`) or alias (`#room:server`). |

## Configuration

The server is configured through environment variables. All are optional — you
can also authenticate at runtime with the `login` tool.

| Variable             | Description |
|----------------------|-------------|
| `MATRIX_HOMESERVER`  | Default homeserver URL, e.g. `https://matrix.org`. |
| `MATRIX_USER`        | Username for automatic login at startup. |
| `MATRIX_PASSWORD`    | Password for automatic login at startup. |
| `MATRIX_DEVICE_NAME` | Device display name (default `matrix-mcp`). |
| `MATRIX_SESSION_FILE`| Path to persist the session (default: `$XDG_STATE_HOME/matrix-mcp/session.json`, falling back to `~/.local/state/matrix-mcp/session.json`). |
| `RUST_LOG`           | Log filter, e.g. `matrix_mcp=debug,matrix_sdk=info`. Logs go to stderr. |

On startup the server tries to restore a saved session; if none exists and
`MATRIX_USER`/`MATRIX_PASSWORD` are set, it performs a password login.

## Build

```sh
cargo build --release
```

The binary is written to `target/release/matrix-mcp`. Default `matrix-sdk`
features are disabled to keep the build lightweight (rustls for TLS, no native
OpenSSL/sqlite).

## Use with an MCP client

The server speaks MCP over stdio. Register it with your client, for example for
Claude Code:

```sh
claude mcp add matrix \
  --env MATRIX_HOMESERVER=https://matrix.org \
  --env MATRIX_USER=alice \
  --env MATRIX_PASSWORD=... \
  -- /path/to/target/release/matrix-mcp
```

Or, equivalently, an entry in an MCP `servers` configuration:

```json
{
  "mcpServers": {
    "matrix": {
      "command": "/path/to/target/release/matrix-mcp",
      "env": {
        "MATRIX_HOMESERVER": "https://matrix.org",
        "MATRIX_USER": "alice",
        "MATRIX_PASSWORD": "..."
      }
    }
  }
}
```

## Limitations

- **End-to-end encryption is not enabled.** Encrypted rooms are listed (and
  flagged via their encryption state), but their message contents cannot be
  decrypted or sent encrypted. This keeps the dependency footprint and build
  small. E2EE support could be added by enabling the `matrix-sdk`
  `e2e-encryption` feature.
- Session state is kept in memory plus a persisted access token; room history is
  fetched on demand rather than cached locally.

## License

MIT
