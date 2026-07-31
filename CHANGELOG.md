# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-07-31

### Added

- **All-in-one macOS DMG** — single installer: Tauri app + bundled Python Pocket Node + production web UI (`scripts/build-all-in-one-dmg.sh` in the org workspace)
- **Auto-start Pocket Node** — release builds spawn bundled `pocket-agent serve` on `127.0.0.1:8787` and open the UI in the window
- **First-run flow** — startup screen while Pocket Node boots, then `/welcome` onboarding (desktop bundle profile)
- **Bundled resources** — `pocket-node` venv, agent source, SDK (Python), and web `dist` embedded in the app
- **Tauri 2** native shell for the Pocket Agent chat and settings UI
- **Dev integration** — `npm run dev` loads [pocket-agent-web-app](https://github.com/pocket-agent/pocket-agent-web-app) on `:5173` with Pocket Node + API worker in separate terminals
- **Desktop icons** — from `pocket-agent/.github/pocket-agent-image.png`

### Notes

- **Release installs** do not require a separate Cloudflare API worker; the UI uses Pocket Node on `:8787` with `AUTH_MODE=none`.
- **DMG artifact:** `Pocket Agent_0.1.0_aarch64.dmg` (Apple Silicon). Output copied to `release/` after a local build.

---

## Repository documents

[README](README.md) | [INSTRUCTIONS](INSTRUCTIONS.md) | **CHANGELOG** | [CONTRIBUTING](CONTRIBUTING.md) | [SECURITY](SECURITY.md) | [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md)
