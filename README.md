# QXP Client - Tauri App

Desktop client for the QXP messaging web app, built with Tauri v2 and TypeScript.

## Installation

Clone with submodules, or initialize them after cloning:

```bash
git submodule update --init --recursive
```

```bash
bun install
```

On NixOS, enter the prepared shell first:

```bash
nix develop
```

If you do not use flakes:

```bash
nix-shell
```

## Development

```bash
bun run dev
```

## Build

```bash
bun run build
```

Platform helpers are also available:

```bash
bun run build:mac
bun run build:win
bun run build:linux
```

## Permissions

The main Tauri capability is declared in `src-tauri/capabilities/default.json`.
It grants the bundled QXP web client access to core Tauri APIs, notification APIs for messaging, and restricted URL opening for QXP, `mailto:` and `tel:` links.

With `"withGlobalTauri": true`, the bundled page can use Tauri guest APIs through `window.__TAURI__` when needed, including notifications.

Native media permissions for macOS are declared in `src-tauri/Info.plist` for camera and microphone access used by calls or voice features in the remote web app. Speaker output does not require a separate Tauri permission.

## NixOS

`shell.nix` and `flake.nix` include the Linux dependencies that Tauri expects on NixOS, including GTK, WebKitGTK 4.1, GLib, `libsoup_3`, `librsvg`, and the GIO networking module setup required by WebKit.

## License

MIT
