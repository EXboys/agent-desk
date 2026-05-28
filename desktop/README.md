# Desktop (Tauri menubar)

**Tauri 2** menubar companion that calls the same Rust core as the CLI (`agent-doctor-core`).

## Features (MVP)

- System tray with **Show**, **Run doctor**, **Quit**
- Small window listing discovered runtimes and company profile status
- Hermes model preset switching and API key status
- Foundation for future repair reports and guided fixes
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

You can use Agent Doctor without the desktop app:

```bash
cargo run -p agent-doctor -- doctor
```
