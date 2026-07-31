<img src=".github/pocket-agent-image.png" width="200" alt="Pocket Agent" align="left"/>

<div>
<h3>Pocket Agent (Desktop)</h3>
<p>
Native <strong>macOS</strong> app (Tauri 2) for Pocket Agent — all-in-one DMG with bundled Pocket Node and React chat UI. Release builds auto-start Pocket Node and open the dashboard (no separate API worker install).
</p>
<a href="https://github.com/pocket-agent/pocket-agent-desktop-app/releases"><img src="https://img.shields.io/badge/Download%20for%20macOS-007ec6?style=flat-square&logo=apple" width="175" alt="Download for macOS"/></a>
</div>

<br/><br/>

<div align="center">

[![Downloads](https://img.shields.io/badge/downloads-GitHub%20Releases-007ec6)](https://github.com/pocket-agent/pocket-agent-desktop-app/releases)
[![Release](https://img.shields.io/github/v/release/pocket-agent/pocket-agent-desktop-app)](https://github.com/pocket-agent/pocket-agent-desktop-app/releases)
[![License](https://img.shields.io/badge/License-MIT-blue)](https://github.com/pocket-agent/pocket-agent-desktop-app/blob/main/LICENSE)
[![macOS](https://img.shields.io/badge/macOS-Apple%20Silicon-blue)](https://github.com/pocket-agent/pocket-agent-desktop-app)

<br/>
<br/>

<img src=".github/screenshot.png" width="824" alt="Pocket Agent desktop" style="border-radius: 5px;"/><br/>

</div>

<hr>

## Features

- **All-in-one DMG** — Pocket Node venv, web UI build, and Tauri shell (see workspace `scripts/build-all-in-one-dmg.sh`)
- **Auto-start** — spawns local API on `:8787` and navigates to the embedded UI on launch
- **Dev mode** — loads [pocket-agent-web-app](https://github.com/pocket-agent/pocket-agent-web-app) from `localhost:5173` with live reload
- **Same UX as browser** — local `AUTH_MODE=none` in the bundled app; dev can use Google OAuth via API worker on `:8788`
- **Icons** — generated from `pocket-agent/.github/pocket-agent-image.png`

## Requirements

- **macOS** (Apple Silicon builds in CI; Intel via local build)
- [Rust](https://rustup.rs) and Node/Bun for development
- Full stack siblings: [pocket-agent](https://github.com/pocket-agent/pocket-agent), [pocket-agent-api-app](https://github.com/pocket-agent/pocket-agent-api-app), [pocket-agent-web-app](https://github.com/pocket-agent/pocket-agent-web-app)

## Install

1. Download the DMG from **[GitHub Releases](https://github.com/pocket-agent/pocket-agent-desktop-app/releases)** or [pocket-agent.pages.dev](https://pocket-agent.pages.dev)
2. Open the DMG and drag **Pocket Agent** to Applications
3. Launch — the app starts Pocket Node and opens the chat UI

## Quick start (dev)

From the **org workspace** root (all sibling repos checked out):

```bash
./scripts/setup-local.sh
./scripts/dev-desktop.sh
```

Or three terminals: `pocket-agent serve`, `npm run dev` in api-app, `npm run dev` in this repo.

## Development

```bash
git clone https://github.com/pocket-agent/pocket-agent-desktop-app.git
cd pocket-agent-desktop-app
npm install
npm run dev
```

Production DMG from workspace:

```bash
../scripts/build-all-in-one-dmg.sh
```

See [docs/ALL_IN_ONE_DMG.md](../docs/ALL_IN_ONE_DMG.md) (workspace) and [INSTRUCTIONS.md](INSTRUCTIONS.md).

## Documentation

| Doc | Description |
|-----|-------------|
| [INSTRUCTIONS.md](INSTRUCTIONS.md) | Desktop app scope |
| [CHANGELOG.md](CHANGELOG.md) | Release history |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute |

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

## License

Pocket Agent Desktop is released under the [MIT License](LICENSE).
