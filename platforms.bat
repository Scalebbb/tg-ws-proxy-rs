@echo off
setlocal enabledelayedexpansion

echo ==========================================
echo   Multi-platform build with cargo-zigbuild
echo ==========================================
echo.

REM Check if cargo-zigbuild is installed
where cargo-zigbuild >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo X cargo-zigbuild not found!
    echo Install it with: cargo install cargo-zigbuild
    pause
    exit /b 1
)

REM Check if zig is installed
where zig >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo X zig not found!
    echo Install it from: https://ziglang.org/download/
    pause
    exit /b 1
)

echo + cargo-zigbuild found
echo + zig found
echo.

REM Create output directory
if not exist dist mkdir dist

echo ============================================
echo Building Linux targets (headless - no GUI)
echo ============================================
echo.

echo Building for: Linux x86_64
cargo zigbuild --release --target x86_64-unknown-linux-musl --no-default-features --features dc-updater
if %ERRORLEVEL% EQU 0 (
    echo + Build successful
    copy target\x86_64-unknown-linux-musl\release\tg-ws-proxy dist\tg-ws-proxy-linux-x86_64 >nul 2>nul
) else (
    echo X Build failed
)
echo.

echo Building for: Linux aarch64
cargo zigbuild --release --target aarch64-unknown-linux-musl --no-default-features --features dc-updater
if %ERRORLEVEL% EQU 0 (
    echo + Build successful
    copy target\aarch64-unknown-linux-musl\release\tg-ws-proxy dist\tg-ws-proxy-linux-aarch64 >nul 2>nul
) else (
    echo X Build failed
)
echo.

echo Building for: Linux armv7
cargo zigbuild --release --target armv7-unknown-linux-musleabihf --no-default-features --features dc-updater
if %ERRORLEVEL% EQU 0 (
    echo + Build successful
    copy target\armv7-unknown-linux-musleabihf\release\tg-ws-proxy dist\tg-ws-proxy-linux-armv7 >nul 2>nul
) else (
    echo X Build failed
)
echo.

echo Building for: Linux mipsel
cargo zigbuild --release --target mipsel-unknown-linux-musl --no-default-features --features dc-updater
if %ERRORLEVEL% EQU 0 (
    echo + Build successful
    copy target\mipsel-unknown-linux-musl\release\tg-ws-proxy dist\tg-ws-proxy-linux-mipsel >nul 2>nul
) else (
    echo X Build failed
)
echo.

echo ============================================
echo Building macOS targets (with GUI)
echo ============================================
echo.

echo Building for: macOS x86_64
cargo zigbuild --release --target x86_64-apple-darwin
if %ERRORLEVEL% EQU 0 (
    echo + Build successful
    copy target\x86_64-apple-darwin\release\tg-ws-proxy dist\tg-ws-proxy-macos-x86_64 >nul 2>nul
) else (
    echo X Build failed
)
echo.

echo Building for: macOS aarch64 (Apple Silicon)
cargo zigbuild --release --target aarch64-apple-darwin
if %ERRORLEVEL% EQU 0 (
    echo + Build successful
    copy target\aarch64-apple-darwin\release\tg-ws-proxy dist\tg-ws-proxy-macos-aarch64 >nul 2>nul
) else (
    echo X Build failed
)
echo.

echo ============================================
echo Building Windows targets (with GUI)
echo ============================================
echo.

echo Building for: Windows x86_64
cargo zigbuild --release --target x86_64-pc-windows-gnu
if %ERRORLEVEL% EQU 0 (
    echo + Build successful
    copy target\x86_64-pc-windows-gnu\release\tg-ws-proxy.exe dist\tg-ws-proxy-windows-x86_64.exe >nul 2>nul
) else (
    echo X Build failed
)
echo.

echo ==========================================
echo   Build complete!
echo ==========================================
echo.
echo Binaries are in the 'dist' folder:
dir /B dist
echo.
pause
