# gba-dvd
GBA program that is just the DVD bounce screen saver

## Setup
Follow the `rust-console/gba` setup (https://github.com/rust-console/gba).

## Building
If you install `cargo-make`, you build and fix the ROM with the Makefile.toml
```
cargo make justrelease
```

Otherwise you can build the ROM file with and fix the ROM manually
```
cargo build --release

arm-none-eabi-objcopy -O binary [RUST_BINARY_NAME] [ROM_NAME].gba
gbafix [ROM_NAME].gba
```