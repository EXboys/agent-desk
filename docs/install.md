# Install Agent Desk

## Prerequisites

- [GitHub CLI](https://cli.github.com/) (`gh`) for downloading release assets
- Or download files manually from [Releases](https://github.com/EXboys/agent-desk/releases)

## CLI

Pick the archive that matches your OS and CPU:

| Asset pattern | Platform |
|---------------|----------|
| `agent-desk-*-macos-arm64.tar.gz` | macOS Apple Silicon |
| `agent-desk-*-macos-x86_64.tar.gz` | macOS Intel |
| `agent-desk-*-linux-x86_64.tar.gz` | Linux x86_64 |
| `agent-desk-*-windows-x86_64.zip` | Windows x86_64 |

### macOS (Apple Silicon)

```bash
gh release download --repo EXboys/agent-desk --pattern 'agent-desk-*-macos-arm64.tar.gz'
tar -xzf agent-desk-*-macos-arm64.tar.gz
chmod +x agent-desk
sudo mv agent-desk /usr/local/bin/
agent-desk doctor
```

### macOS (Intel)

```bash
gh release download --repo EXboys/agent-desk --pattern 'agent-desk-*-macos-x86_64.tar.gz'
tar -xzf agent-desk-*-macos-x86_64.tar.gz
chmod +x agent-desk
sudo mv agent-desk /usr/local/bin/
agent-desk doctor
```

### Linux

```bash
gh release download --repo EXboys/agent-desk --pattern 'agent-desk-*-linux-x86_64.tar.gz'
tar -xzf agent-desk-*-linux-x86_64.tar.gz
chmod +x agent-desk
sudo mv agent-desk /usr/local/bin/
agent-desk doctor
```

### Windows (PowerShell)

```powershell
gh release download --repo EXboys/agent-desk --pattern "agent-desk-*-windows-x86_64.zip"
Expand-Archive agent-desk-*-windows-x86_64.zip -DestinationPath .
Move-Item .\agent-desk.exe "$env:LOCALAPPDATA\Programs\agent-desk\"
```

## Desktop (menubar app)

After a release is published, download the desktop bundle for your platform from the same GitHub release:

- **macOS**: `.dmg`
- **Windows**: `.msi` or `.exe` setup
- **Linux**: `.AppImage` or `.deb` (when built by Tauri)

```bash
# List desktop assets for the latest release
gh release view --repo EXboys/agent-desk --json assets --jq '.assets[].name'
gh release download --repo EXboys/agent-desk --pattern '*.dmg'
```

## Build from source

```bash
# CLI
cargo install --path cli --locked

# Desktop dev
cd desktop
npm install
npm run tauri dev
```

## Create a release (maintainers)

Push a version tag to trigger `.github/workflows/release.yml`:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This builds CLI archives for all platforms and attaches Tauri desktop installers to the GitHub release.
