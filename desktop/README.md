# Desktop (Tauri menubar)

**Tauri 2** menubar companion that calls the same Rust core as the CLI (`agent-desk-core`).

## Features (MVP)

- System tray with **Show**, **Run doctor**, **Quit**
- Small window listing discovered runtimes and company profile status
- No separate business logic in the TypeScript UI layer

## Develop

```bash
cd desktop
npm install
npm run tauri dev
```

## Build

```bash
cd desktop
npm run tauri build
```

## CLI-only workflow

You can use Agent Desk without the desktop app:

```bash
cargo run -p agent-desk -- doctor
```
