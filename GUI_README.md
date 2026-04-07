# GUI Mode

This project now includes a graphical user interface built with egui.

## Features

- Real-time connection statistics
- Live log viewer
- Configuration display
- One-click Telegram proxy link copying
- Silent operation (no console window on Windows)
- Automatic file-based logging to `logs/` directory

## Building

### With GUI (default):
```bash
cargo build --release
```

### Without GUI (headless mode):
```bash
cargo build --release --no-default-features
```

## Running

Simply run the executable:
```bash
./target/release/tg-ws-proxy
```

On Windows, the application will run without showing a console window. All logs are written to the `logs/` directory.

## Logs

Logs are automatically saved to:
- `logs/tg-ws-proxy.log.YYYY-MM-DD`

A new log file is created each day, and old logs are preserved.

## Command Line Arguments

All command-line arguments work the same as before:
```bash
tg-ws-proxy --host 0.0.0.0 --port 1443 --secret YOUR_SECRET
```

See the main README.md for full documentation of available options.
