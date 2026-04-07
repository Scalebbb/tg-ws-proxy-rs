# Кросс-компиляция с cargo-zigbuild

## Установка инструментов

### 1. Установите Zig

**Windows:**
```powershell
# Скачайте с https://ziglang.org/download/
# Или через Scoop:
scoop install zig

# Или через Chocolatey:
choco install zig
```

**Linux:**
```bash
# Ubuntu/Debian
wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
tar -xf zig-linux-x86_64-0.11.0.tar.xz
sudo mv zig-linux-x86_64-0.11.0 /opt/zig
echo 'export PATH=$PATH:/opt/zig' >> ~/.bashrc
source ~/.bashrc

# Arch Linux
sudo pacman -S zig

# Fedora
sudo dnf install zig
```

**macOS:**
```bash
brew install zig
```

### 2. Установите cargo-zigbuild

```bash
cargo install cargo-zigbuild
```

### 3. Добавьте targets (опционально)

```bash
# Linux targets
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
rustup target add armv7-unknown-linux-musleabihf
rustup target add mipsel-unknown-linux-musl

# macOS targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Windows targets
rustup target add x86_64-pc-windows-gnu
```

## Сборка

### Автоматическая сборка всех платформ

**Linux/macOS:**
```bash
chmod +x platforms.sh
./platforms.sh
```

**Windows:**
```cmd
platforms.bat
```

### Ручная сборка для конкретной платформы

**Linux x86_64 (headless):**
```bash
cargo zigbuild --release --target x86_64-unknown-linux-musl --no-default-features --features dc-updater
```

**Linux ARM64 (headless):**
```bash
cargo zigbuild --release --target aarch64-unknown-linux-musl --no-default-features --features dc-updater
```

**macOS x86_64 (с GUI):**
```bash
cargo zigbuild --release --target x86_64-apple-darwin
```

**macOS ARM64 / Apple Silicon (с GUI):**
```bash
cargo zigbuild --release --target aarch64-apple-darwin
```

**Windows x86_64 (с GUI):**
```bash
cargo zigbuild --release --target x86_64-pc-windows-gnu
```

## Поддерживаемые платформы

### Linux (headless - без GUI)
- ✅ x86_64 (Intel/AMD 64-bit)
- ✅ aarch64 (ARM 64-bit, Raspberry Pi 4+)
- ✅ armv7 (ARM 32-bit, Raspberry Pi 2/3)
- ✅ mipsel (MIPS Little Endian, роутеры)

### macOS (с GUI)
- ✅ x86_64 (Intel Mac)
- ✅ aarch64 (Apple Silicon M1/M2/M3)

### Windows (с GUI)
- ✅ x86_64 (64-bit)

## Особенности сборки

### Linux targets
- Собираются **без GUI** (headless)
- Используют musl libc (статическая линковка)
- Не требуют системных библиотек
- Идеальны для серверов и встраиваемых систем

### macOS targets
- Собираются **с GUI**
- Требуют macOS для запуска
- Universal binary можно создать с помощью `lipo`

### Windows targets
- Собираются **с GUI**
- Используют GNU toolchain
- Тихий режим (без консоли) в release

## Выходные файлы

После сборки бинарники находятся в папке `dist/`:

```
dist/
├── tg-ws-proxy-linux-x86_64      # Linux 64-bit
├── tg-ws-proxy-linux-aarch64     # Linux ARM64
├── tg-ws-proxy-linux-armv7       # Linux ARM32
├── tg-ws-proxy-linux-mipsel      # Linux MIPS
├── tg-ws-proxy-macos-x86_64      # macOS Intel
├── tg-ws-proxy-macos-aarch64     # macOS Apple Silicon
└── tg-ws-proxy-windows-x86_64.exe # Windows 64-bit
```

## Размеры бинарников

Примерные размеры после strip:

| Платформа | С GUI | Без GUI |
|-----------|-------|---------|
| Linux x86_64 | - | ~15 MB |
| Linux ARM64 | - | ~14 MB |
| Linux ARMv7 | - | ~13 MB |
| Linux MIPS | - | ~16 MB |
| macOS x86_64 | ~25 MB | - |
| macOS ARM64 | ~23 MB | - |
| Windows x86_64 | ~20 MB | - |

## Оптимизация размера

Для еще меньших бинарников используйте профиль `release-small`:

```bash
cargo zigbuild --profile release-small --target x86_64-unknown-linux-musl --no-default-features --features dc-updater
```

Дополнительная оптимизация:
```bash
# После сборки
strip target/x86_64-unknown-linux-musl/release/tg-ws-proxy

# Или с UPX (сжатие)
upx --best --lzma target/x86_64-unknown-linux-musl/release/tg-ws-proxy
```

## Проверка бинарников

### Linux
```bash
file dist/tg-ws-proxy-linux-x86_64
ldd dist/tg-ws-proxy-linux-x86_64  # Должно показать "not a dynamic executable"
```

### macOS
```bash
file dist/tg-ws-proxy-macos-x86_64
otool -L dist/tg-ws-proxy-macos-x86_64
```

### Windows
```bash
file dist/tg-ws-proxy-windows-x86_64.exe
```

## Тестирование

### На той же платформе
```bash
./dist/tg-ws-proxy-linux-x86_64 --help
```

### На другой платформе (эмуляция)
```bash
# QEMU для ARM
qemu-aarch64 ./dist/tg-ws-proxy-linux-aarch64 --help

# Wine для Windows на Linux
wine ./dist/tg-ws-proxy-windows-x86_64.exe --help
```

## Устранение проблем

### Ошибка: "zig not found"
```bash
# Проверьте установку
zig version

# Добавьте в PATH
export PATH=$PATH:/path/to/zig
```

### Ошибка: "cargo-zigbuild not found"
```bash
cargo install cargo-zigbuild
```

### Ошибка при сборке macOS на Linux
```bash
# Убедитесь, что установлен Xcode SDK (для кросс-компиляции)
# Или соберите на macOS
```

### Ошибка: "linker not found"
```bash
# Проверьте .cargo/config.toml
# Убедитесь, что rust-lld установлен
rustup component add llvm-tools-preview
```

### GUI не работает на Linux
```bash
# Linux targets собираются без GUI (headless)
# Это нормально для серверов
# Для GUI на Linux соберите с --features gui
cargo zigbuild --release --target x86_64-unknown-linux-gnu --features gui
```

## CI/CD интеграция

### GitHub Actions

```yaml
name: Build

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Install Zig
        run: |
          wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
          tar -xf zig-linux-x86_64-0.11.0.tar.xz
          echo "$PWD/zig-linux-x86_64-0.11.0" >> $GITHUB_PATH
          
      - name: Install cargo-zigbuild
        run: cargo install cargo-zigbuild
        
      - name: Build all platforms
        run: ./platforms.sh
        
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: binaries
          path: dist/*
```

## Дополнительные ресурсы

- [cargo-zigbuild GitHub](https://github.com/rust-cross/cargo-zigbuild)
- [Zig Documentation](https://ziglang.org/documentation/master/)
- [Rust Cross Compilation](https://rust-lang.github.io/rustup/cross-compilation.html)

## FAQ

**Q: Почему Linux собирается без GUI?**  
A: Серверы обычно не имеют графического окружения. Для GUI на Linux используйте `--features gui`.

**Q: Можно ли собрать для Android/iOS?**  
A: Да, но требуются дополнительные targets и настройки.

**Q: Работает ли на Raspberry Pi?**  
A: Да! Используйте `armv7` для Pi 2/3 или `aarch64` для Pi 4+.

**Q: Можно ли собрать все на Windows?**  
A: Да, используйте `platforms.bat`.

**Q: Как уменьшить размер бинарника?**  
A: Используйте профиль `release-small` и `strip`/`upx`.
