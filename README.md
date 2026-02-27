# `helium`

> A GPU-accelerated math object viewer built in Rust

## Technology Stack

| Component  | Crate(s)                     |
| ---------- | ---------------------------- |
| Windowing  | `winit`                      |
| GPU        | `wgpu`                       |
| UI overlay | `egui`                       |
| Math       | `glam`, `rustfft`, `ndarray` |

## Cross-Platform Targets

`helium` now has CI coverage for these targets:

- Desktop build: `x86_64-unknown-linux-gnu`
- Desktop build: `x86_64-pc-windows-msvc`
- Desktop build: `x86_64-apple-darwin`
- Desktop build: `aarch64-apple-darwin`
- Mobile compile check: `aarch64-apple-ios`
- Mobile compile check: `aarch64-apple-ios-sim`
- Mobile compile check: `aarch64-linux-android`
- Mobile compile check: `armv7-linux-androideabi`
- Mobile compile check: `x86_64-linux-android`

The workflow is at `.github/workflows/build-matrix.yml`.

## Local Build Commands

Install targets first:

```bash
rustup target add \
	x86_64-unknown-linux-gnu \
	x86_64-pc-windows-msvc \
	x86_64-apple-darwin \
	aarch64-apple-darwin \
	aarch64-apple-ios \
	aarch64-apple-ios-sim \
	aarch64-linux-android \
	armv7-linux-androideabi \
	x86_64-linux-android
```

Desktop release builds:

```bash
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
```

Mobile compile checks:

```bash
cargo check --target aarch64-apple-ios
cargo check --target aarch64-apple-ios-sim
cargo check --target aarch64-linux-android
cargo check --target armv7-linux-androideabi
cargo check --target x86_64-linux-android
```

## Mobile Packaging Notes

- iOS packaging/signing must be done from Xcode (or `xcodebuild`) with an app host project.
- Android packaging/signing must be done with Android SDK/NDK + Gradle.
- This repository is now structured so the core viewer runtime can be reused by those native app hosts.
