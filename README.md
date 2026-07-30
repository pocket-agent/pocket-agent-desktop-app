# pocket-agent-desktop-app

Tauri 2 native shell for [pocket-agent-web-app](../pocket-agent-web-app/).

Part of the open-source **[Pocket Agent](https://github.com/pocket-agent)** ecosystem · **v0.1.0**

## What's included (0.1.0)

- **Tauri 2** window loading the web UI (`localhost:5173` in dev)
- **Dev integration** — runs `bun run dev` in `pocket-agent-web-app` automatically
- **Same auth flow** as browser — Google OAuth via embedded webview

## Prerequisites

- [Rust](https://rustup.rs) (`cargo`)
- Bun, Pocket Node, and API worker (see workspace setup)

## Production DMG (all-in-one)

From the **workspace root** (not this folder alone):

```bash
../scripts/build-all-in-one-dmg.sh
```

Ships Pocket Agent with bundled Pocket Node and auto-start on launch. See [docs/ALL_IN_ONE_DMG.md](../docs/ALL_IN_ONE_DMG.md).

## Quick start (dev)

From workspace root:

```bash
./scripts/setup-local.sh
./scripts/dev-desktop.sh
```

| Terminal | Directory | Command |
|----------|-----------|---------|
| 1 | `pocket-agent/` | `pocket-agent serve` |
| 2 | `pocket-agent-api-app/` | `npm run dev` |
| 3 | `pocket-agent-desktop-app/` | `npm run dev` |

## Docs

[INSTRUCTIONS.md](INSTRUCTIONS.md) · [CHANGELOG.md](CHANGELOG.md)
