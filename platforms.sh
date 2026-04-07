echo "Start build"
cargo zigbuild --release --target x86_64-unknown-linux-musl
cargo zigbuild --release --target aarch64-unknown-linux-musl
cargo zigbuild --release --target armv7-unknown-linux-musleabihf
cargo zigbuild --release --target mipsel-unknown-linux-musl
cargo zigbuild --release --target x86_64-apple-darwin
cargo zigbuild --release --target aarch64-apple-darwin
cargo zigbuild --release --target x86_64-pc-windows-gnu
echo "build its end!"
read -p "Press enter to exit" </dev/tty
