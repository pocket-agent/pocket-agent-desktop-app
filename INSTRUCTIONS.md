# Agent instructions — pocket-agent-desktop

**Scope:** Tauri wrapper only. Global rules: [../INSTRUCTIONS.md](../INSTRUCTIONS.md).

## Responsibilities

- Native window hosting the web UI
- `src-tauri/tauri.conf.json` — paths to `../pocket-agent-web-app`

## Do not duplicate

- React components → `../pocket-agent-web-app/src/`
- API / auth logic → web + worker repos

## OAuth note

Release builds may need extra Google Cloud redirect configuration for non-browser contexts. Test sign-in before shipping.

## Related

- [../docs/GOOGLE_OAUTH.md](../docs/GOOGLE_OAUTH.md)
- [../pocket-agent-web-app/INSTRUCTIONS.md](../pocket-agent-web-app/INSTRUCTIONS.md)
