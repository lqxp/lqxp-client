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

## iOS

iOS builds require macOS with the full Xcode app installed.

```bash
nix-shell
bun run ios:build --export-method development
```

For development on a simulator or device:

```bash
nix-shell
bun run ios:dev -- --open
```

## Android

Android builds are handled by `scripts/build-android.sh` through the package script:

```bash
bun run build:android
```

The script automatically enters the Nix development shell with `nix develop` when it is not already running inside Nix. It also prepares the Android SDK/NDK environment, Rust Android targets, and Tauri build dependencies.

By default, `bun run build:android` builds a debug APK for `aarch64`:

```bash
bun run build:android
# equivalent default args: --debug --apk --target aarch64
```

Debug APKs are signed with Android's debug key and are installable on development devices, but they are not suitable for release distribution.

To build a release APK:

```bash
bun run build:android -- --apk --target aarch64
```

Release APK signing is configured through `src-tauri/gen/android/keystore.properties`, which is ignored by git. The Gradle project reads these values from that file or from the environment:

```properties
ANDROID_KEYSTORE_PATH=/absolute/path/to/lqxp-release.jks
ANDROID_KEYSTORE_PASSWORD=change-me
ANDROID_KEY_ALIAS=lqxp
ANDROID_KEY_PASSWORD=change-me
```

The build script can create a local release keystore and configure signing automatically:

```bash
LQXP_ANDROID_CREATE_KEYSTORE=1 ANDROID_KEYSTORE_PASSWORD='change-me' bun run build:android -- --apk --target aarch64
```

By default, the generated keystore is stored at:

```text
~/.config/lqxp-client/lqxp-release.jks
```

For subsequent signed release builds, provide the same keystore password:

```bash
ANDROID_KEYSTORE_PASSWORD='change-me' bun run build:android -- --apk --target aarch64
```

You can also point to an existing keystore:

```bash
ANDROID_KEYSTORE_PATH=/absolute/path/to/release.jks \
ANDROID_KEYSTORE_PASSWORD='change-me' \
ANDROID_KEY_ALIAS=lqxp \
ANDROID_KEY_PASSWORD='change-me' \
bun run build:android -- --apk --target aarch64
```

Expected APK output locations include:

```text
src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk
src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk
```

If release signing is not configured, Gradle may produce an unsigned release artifact such as:

```text
src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk
```

If Gradle/Tauri fails with a WebSocket or IPC error such as `failed to read CLI options` or `Connection refused`, stop stale Gradle daemons and rebuild:

```bash
src-tauri/gen/android/gradlew --project-dir src-tauri/gen/android --stop
bun run build:android
```

The script also disables the Gradle daemon for Android builds because Gradle daemons can keep stale Tauri IPC environment variables.

The GitHub workflow for simulator builds and signed IPA builds is documented in [docs/tauri-ios.md](docs/tauri-ios.md).

## Permissions

The main Tauri capability is declared in `src-tauri/capabilities/default.json`.
It grants the bundled QXP web client access to core Tauri APIs, notification APIs for messaging, and restricted URL opening for QXP, `mailto:` and `tel:` links.

With `"withGlobalTauri": true`, the bundled page can use Tauri guest APIs through `window.__TAURI__` when needed, including notifications.

Native media permissions for macOS are declared in `src-tauri/Info.plist` for camera and microphone access used by calls or voice features in the remote web app. Speaker output does not require a separate Tauri permission.

## NixOS

`shell.nix` and `flake.nix` include the Linux dependencies that Tauri expects on NixOS, including GTK, WebKitGTK 4.1, GLib, `libsoup_3`, `librsvg`, and the GIO networking module setup required by WebKit.

## License

MIT
