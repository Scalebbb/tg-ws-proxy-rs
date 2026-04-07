[troubleshooting](https://github.com/scalebbb/tg-ws-proxy-rs/other/TROUBLESHOOTING_RU.md) | [Old readme](https://github.com/scalebbb/tg-ws-proxy-rs/other/oldREADME.md)

</div>

> [!WARNING]
> ### Original developer - flowseal
> https://github.com/Flowseal/tg-ws-proxy

<div align="center">

# <img src="https://github.com/Scalebbb/tg-ws-proxy-rs/blob/main/icon.ico" width="20" height="20"> [TG-WS-PROXY](https://github.com/Flowseal/tg-ws-proxy) on rust

# Usage


```
tg-ws-proxy [OPTIONS]
```

| Flag | Default | Description |
|---|---|---|
| `--port <PORT>` | `1443` | Listen port |
| `--host <HOST>` | `127.0.0.1` | Listen address |
| `--link-ip <IP>` | auto-detected | IP shown in the `tg://` link (see [Router deployment](#router-deployment)) |
| `--secret <HEX>` | random | 32 hex-char MTProto secret |
| `--dc-ip <DC:IP>` | DC2 + DC4 | Target IP per DC (repeatable) |
| `--buf-kb <KB>` | `256` | Socket buffer size |
| `--pool-size <N>` | `4` | Pre-warmed WS connections per DC |
| `--mtproto-proxy <HOST:PORT:SECRET>` | — | Upstream MTProto proxy fallback (repeatable) |
| `-q / --quiet` | off | Suppress all log output |
| `-v / --verbose` | off | Debug logging |
| `--danger-accept-invalid-certs` | off | Skip TLS verification |

Every flag has a matching environment variable (`TG_PORT`, `TG_HOST`,
`TG_SECRET`, `TG_BUF_KB`, `TG_POOL_SIZE`, `TG_QUIET`, `TG_VERBOSE`,
`TG_SKIP_TLS_VERIFY`, `TG_LINK_IP`, `TG_MTPROTO_PROXY`)

```bash
# Standard run (random secret, DC 2 + 4)
tg-ws-proxy

# Custom port and extra DCs
tg-ws-proxy --port 9050 --dc-ip 1:149.154.175.205 --dc-ip 2:149.154.167.220

# With upstream MTProto proxy fallback
tg-ws-proxy --mtproto-proxy proxy.example.com:443:abcdef1234567890abcdef1234567890

# Multiple upstream proxies (tried in order until one succeeds)
tg-ws-proxy \
  --mtproto-proxy proxy.example.com:443:abcdef1234567890abcdef1234567890 \
  --mtproto-proxy other.example.net:8888:deadbeef01234567deadbeef01234567

# Router deployment: listen on all interfaces, let all LAN devices use the proxy
tg-ws-proxy --host 0.0.0.0

# Verbose logging
tg-ws-proxy -v

# All options via environment variables (useful for Docker / systemd)
TG_PORT=1443 TG_SECRET=deadbeef... tg-ws-proxy
```



# How to install

1. Install from [releases](https://github.com/Scalebbb/tg-ws-proxy-rs/releases)

2. Build from src 
    cargo build --release in your terminal
    build for all platforms:
    !(you need install cargo-zigbuild and zig)!
    ```bash 
    platforms.sh 
    platforms.bat # for windows
    ```
    or 
    ```bash
    cargo zigbuild --release --target x86_64-unknown-linux-musl       # Linux x86-64 (musl static)
    cargo zigbuild --release --target aarch64-unknown-linux-musl      # Linux / OpenWrt ARM64
    cargo zigbuild --release --target armv7-unknown-linux-musleabihf  # OpenWrt ARMv7
    cargo zigbuild --release --target mipsel-unknown-linux-musl       # OpenWrt MIPS LE
    cargo zigbuild --release --target x86_64-apple-darwin             # macOS Intel
    cargo zigbuild --release --target aarch64-apple-darwin            # macOS Apple Silicon
    cargo zigbuild --release --target x86_64-pc-windows-gnu           # Windows x86-64
    ```


# Support

[Support only original developer!](https://github.com/Flowseal/tg-ws-proxy)
