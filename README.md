# Rust Arduino Elegoo

## Description

This repository contains the necessary Rust code to go through the [Elegoo UNO Project Super Starter Kit Tutorials](https://www.elegoo.com/blogs/arduino-projects/elegoo-uno-project-super-starter-kit-tutorial).

The [Elegoo UNO R3](./elegoo_uno_datasheet.pdf) is really just an Arduino Uno board, which is based on the ATmega328p architecture.

## Prerequisites

- [Install the compiler](https://book.avr-rust.com/002-installing-the-compiler.html)
- [Install required 3rd party tools](https://book.avr-rust.com/002.1-installing-required-third-party-tools.html)

**Archlinux setup**

```bash
# install tools
$ pacman -S rustup arduino-avr-core

# configure rust
$ rustup toolchain install nightly
$ rustup component add --toolchain nightly rust-src

# add user to `uucp` group to be able to write to serial device
# you will need to logout & login for change to take effect
$ usermod -aG uucp <username>

# compile and flash examples/
# you might have to change the `SERIAL_PORT` in the `Makefile`
$ rustup override set nightly
$ make build EXAMPLE=<example>
$ make flash EXAMPLE=<example>
```

## Cargo build

The flash memory is `32 KB`, of which `0.5 KB` is used by the bootloader. We need executables that can fit in that space.

I needed to add this to the `Cargo.toml` file so the compilation would work:

```toml
[profile.release]
panic = "abort"
codegen-units = 1
debug = false
lto = true
opt-level = "z"
```

# Ressources

- [Elegoo tutorials](https://www.elegoo.com/pages/arduino-kits-support-files)
- [Arduino Built-In Examples](https://www.arduino.cc/en/Tutorial/BuiltInExamples)
- [avrdude tutorial](http://ladyada.net/learn/avr/avrdude.html)
- [creativcoder blog post](https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0)

## Archwiki

- [Arduino](https://wiki.archlinux.org/index.php/Arduino)
- [AVR](https://wiki.archlinux.org/index.php/AVR)

## Github

- [The AVR-Rust project](https://github.com/avr-rust)
- [Rust AVR book](https://book.avr-rust.com/)
- [awesome-avr-rust](https://github.com/avr-rust/awesome-avr-rust)
- [avr-hal](https://github.com/Rahix/avr-hal)

## Rust

- [Cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
