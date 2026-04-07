# Сборка для всех платформ

## 🚀 Быстрый старт

### Автоматическая сборка

**Linux/macOS:**
```bash
chmod +x platforms.sh
./platforms.sh
```

**Windows:**
```cmd
platforms.bat
```

**Make:**
```bash
make build-all
```

Все бинарники будут в папке `dist/`.

## 📦 Что будет собрано

### Linux (headless - без GUI)
- ✅ `tg-ws-proxy-linux-x86_64` - Intel/AMD 64-bit
- ✅ `tg-ws-proxy-linux-aarch64` - ARM 64-bit (Raspberry Pi 4+)
- ✅ `tg-ws-proxy-linux-armv7` - ARM 32-bit (Raspberry Pi 2/3)
- ✅ `tg-ws-proxy-linux-mipsel` - MIPS (роутеры)

### macOS (с GUI)
- ✅ `tg-ws-proxy-macos-x86_64` - Intel Mac
- ✅ `tg-ws-proxy-macos-aarch64` - Apple Silicon (M1/M2/M3)

### Windows (с GUI)
- ✅ `tg-ws-proxy-windows-x86_64.exe` - 64-bit

## 🛠️ Требования

### 1. Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Zig
**Windows (Scoop):**
```powershell
scoop install zig
```

**Linux:**
```bash
wget https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz
tar -xf zig-linux-x86_64-0.11.0.tar.xz
sudo mv zig-linux-x86_64-0.11.0 /opt/zig
export PATH=$PATH:/opt/zig
```

**macOS:**
```bash
brew install zig
```

### 3. cargo-zigbuild
```bash
cargo install cargo-zigbuild
```

## 📋 Ручная сборка

### Для конкретной платформы

**Linux x86_64:**
```bash
cargo zigbuild --release --target x86_64-unknown-linux-musl --no-default-features --features dc-updater
```

**macOS ARM64:**
```bash
cargo zigbuild --release --target aarch64-apple-darwin
```

**Windows:**
```bash
cargo zigbuild --release --target x86_64-pc-windows-gnu
```

### С помощью Make

```bash
# Linux x86_64
make build-linux-x64

# macOS ARM64
make build-macos-arm64

# Windows
make build-windows-x64
```

## 📊 Размеры бинарников

| Платформа | Размер | Особенности |
|-----------|--------|-------------|
| Linux x86_64 | ~15 MB | Headless, статическая линковка |
| Linux ARM64 | ~14 MB | Headless, статическая линковка |
| Linux ARMv7 | ~13 MB | Headless, статическая линковка |
| macOS x86_64 | ~25 MB | С GUI |
| macOS ARM64 | ~23 MB | С GUI |
| Windows x86_64 | ~20 MB | С GUI, без консоли |

## 🎯 Особенности сборки

### Linux
- **Без GUI** - идеально для серверов
- **Статическая линковка** - не требует системных библиотек
- **musl libc** - работает везде
- **Автообновление DC IP** - включено

### macOS
- **С GUI** - полнофункциональный интерфейс
- **Автообновление DC IP** - включено
- **Universal binary** - можно создать с `lipo`

### Windows
- **С GUI** - полнофункциональный интерфейс
- **Без консоли** - тихий режим в release
- **Автообновление DC IP** - включено

## 🔧 Конфигурация

### Изменить features

**Только автообновление (без GUI):**
```bash
cargo zigbuild --release --target TARGET --no-default-features --features dc-updater
```

**Только GUI (без автообновления):**
```bash
cargo zigbuild --release --target TARGET --no-default-features --features gui
```

**Минимальная версия:**
```bash
cargo zigbuild --release --target TARGET --no-default-features
```

### Оптимизация размера

```bash
# Используйте профиль release-small
cargo zigbuild --profile release-small --target TARGET

# Дополнительно strip
strip target/TARGET/release/tg-ws-proxy

# Сжатие с UPX
upx --best --lzma target/TARGET/release/tg-ws-proxy
```

## 📦 Создание релиза

### Автоматически

```bash
chmod +x release.sh
./release.sh 1.0.7
```

Создаст:
- Папку `release-1.0.7/` с бинарниками
- Архивы `.tar.gz` для Linux/macOS
- Архивы `.zip` для Windows
- Включает документацию

### Вручную

```bash
# Собрать все платформы
./platforms.sh

# Создать архивы
cd dist
tar -czf tg-ws-proxy-linux-x86_64.tar.gz tg-ws-proxy-linux-x86_64
zip tg-ws-proxy-windows-x86_64.zip tg-ws-proxy-windows-x86_64.exe
```

## 🚀 CI/CD

### GitHub Actions

Автоматическая сборка настроена в `.github/workflows/build.yml`:

- ✅ Сборка при push/PR
- ✅ Сборка всех платформ
- ✅ Загрузка артефактов
- ✅ Создание релиза при теге

**Создать релиз:**
```bash
git tag v1.0.7
git push origin v1.0.7
```

GitHub Actions автоматически:
1. Соберет все платформы
2. Создаст релиз
3. Загрузит бинарники

## 🧪 Тестирование

### На той же платформе
```bash
./dist/tg-ws-proxy-linux-x86_64 --version
./dist/tg-ws-proxy-linux-x86_64 --help
```

### Кросс-платформенное (эмуляция)

**QEMU для ARM:**
```bash
# Установите QEMU
sudo apt install qemu-user-static

# Запустите ARM бинарник
qemu-aarch64-static ./dist/tg-ws-proxy-linux-aarch64 --help
```

**Wine для Windows на Linux:**
```bash
# Установите Wine
sudo apt install wine64

# Запустите Windows бинарник
wine ./dist/tg-ws-proxy-windows-x86_64.exe --help
```

## 📝 Проверка бинарников

### Linux
```bash
# Тип файла
file dist/tg-ws-proxy-linux-x86_64

# Зависимости (должно быть "not a dynamic executable")
ldd dist/tg-ws-proxy-linux-x86_64

# Размер
ls -lh dist/tg-ws-proxy-linux-x86_64
```

### macOS
```bash
# Тип файла
file dist/tg-ws-proxy-macos-x86_64

# Зависимости
otool -L dist/tg-ws-proxy-macos-x86_64

# Архитектура
lipo -info dist/tg-ws-proxy-macos-x86_64
```

### Windows
```bash
# Тип файла
file dist/tg-ws-proxy-windows-x86_64.exe

# Размер
ls -lh dist/tg-ws-proxy-windows-x86_64.exe
```

## ⚠️ Устранение проблем

### "zig not found"
```bash
# Проверьте установку
zig version

# Добавьте в PATH
export PATH=$PATH:/path/to/zig
```

### "cargo-zigbuild not found"
```bash
cargo install cargo-zigbuild
```

### Ошибка линковки
```bash
# Проверьте .cargo/config.toml
# Установите llvm-tools
rustup component add llvm-tools-preview
```

### Сборка macOS на Linux не работает
```bash
# Это нормально для некоторых features
# Соберите на macOS или используйте CI/CD
```

### GUI не работает на Linux
```bash
# Linux targets собираются без GUI (headless)
# Это нормально для серверов
# Для GUI используйте --features gui
```

## 📚 Дополнительные ресурсы

- [CROSS_COMPILE_RU.md](CROSS_COMPILE_RU.md) - Подробная документация
- [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild)
- [Zig](https://ziglang.org/)

## 💡 Советы

1. **Используйте кэш** - cargo-zigbuild кэширует сборки
2. **Параллельная сборка** - используйте `-j` для ускорения
3. **Профили** - используйте `release-small` для меньших бинарников
4. **CI/CD** - автоматизируйте сборку через GitHub Actions
5. **Тестируйте** - проверяйте бинарники перед релизом

## ✅ Готово!

Теперь вы можете собирать прокси для всех платформ одной командой:

```bash
./platforms.sh
```

Все бинарники будут в папке `dist/` готовые к распространению!
