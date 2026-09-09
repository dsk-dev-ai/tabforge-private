# TabForge

Ultra-lightweight multi-tab browser recorder.

Features:
- Record multiple browser tabs simultaneously
- Save every tab as a separate MP4
- 1080p recording
- 30–60 FPS
- Low RAM usage
- Intel QuickSync support
- Chrome/Edge/Brave support

Architecture:

Browser Tab
→ Extension
→ Stream
→ Rust Workers
→ FFmpeg
→ Separate MP4 outputs

## Testing & CI

Unit tests cover the pure IPC/recording-session logic (tab → output filename
sanitization, default resolution mapping, and payload-to-session expansion).

```sh
cd desktop-app/src-tauri
cargo test
```

CI (`.github/workflows/ci.yml`) runs `cargo test` on every push to `main`.
