# Agents
Information about AI agents or autonomous scripts operating on this project should be documented here.

## Basic Facts
- **App Name**: Genesis (Initializer)
- **Implementation**: `native/` — a Rust + `egui`/`eframe` desktop app (no webview).
- **Functionality**: A desktop app that functions as a routine and checklist manager (e.g., Calibration, Surveillance, Quick job).
- **Build**: `cargo build --release --manifest-path native/Cargo.toml`; `run_genesis.bat` launches `native/target/release/genesis.exe`.

## Rules
- All static assets (fonts, sound effects, images) must be stored locally in the `native/assets` folder. Do not use external/online links to avoid network dependency issues on startup.
