# Build Instructions

## Prerequisites

- Rust toolchain (install from https://rustup.rs/)
- On Windows: Visual Studio Build Tools or MinGW

## Building

### GUI Version (Default)
```bash
cargo build --release
```

The executable will be in `target/release/tg-ws-proxy.exe` (Windows) or `target/release/tg-ws-proxy` (Linux/Mac).

### Headless Version (No GUI)
```bash
cargo build --release --no-default-features
```

## Running

### Windows
Double-click the executable or run from command line:
```bash
.\target\release\tg-ws-proxy.exe --host 0.0.0.0 --port 1443 --secret YOUR_SECRET
```

The GUI version will run silently without a console window. Check the `logs/` folder for log files.

### Linux/Mac
```bash
./target/release/tg-ws-proxy --host 0.0.0.0 --port 1443 --secret YOUR_SECRET
```

## Features

### GUI Mode
- Visual interface with real-time statistics
- Live log viewer
- Easy copy of Telegram proxy link
- Silent operation on Windows (no console)
- Automatic logging to files

### Headless Mode
- Runs in background
- All output goes to log files in `logs/` directory
- Lower resource usage
- Ideal for servers

## Log Files

Logs are saved to:
```
logs/tg-ws-proxy.log.YYYY-MM-DD
```

A new log file is created daily. Old logs are kept for reference.

## Troubleshooting

### Windows: Console window still appears
Make sure you're building in release mode:
```bash
cargo build --release
```

Debug builds will show the console.

### GUI doesn't start
Check the log files in the `logs/` directory for error messages.

### Port already in use
Change the port with `--port` argument:
```bash
tg-ws-proxy --port 8443
```
