# OLED ESP32 Rust Project

This project demonstrates how to use an SSD1306 OLED display with an ESP32 and Rust. It includes code to initialize the display and show animated text. It also provides instructions for setting up the environment and flashing your ESP32 with the built binary.

## Prerequisites

Before building and flashing the project, ensure you have the following tools installed:


- [ldproxy](https://github.com/esp-rs/ldproxy)  
  ```bash
  cargo install ldproxy
  ```
- [espup](https://github.com/esp-rs/espup)  
  ```bash
  cargo install espup
  ```
- [espflash](https://github.com/esp-rs/espflash)  
  ```bash
  cargo install espflash
  ```
- [cargo-espflash](https://github.com/esp-rs/espflash) (if needed)  
  ```bash
  cargo install cargo-espflash
  ```

Ensure you have an appropriate Rust toolchain for ESP32 development. If not, use `espup` to install the necessary toolchain:

```bash
espup install
```

## Building the Project

Use Cargo to build the project:

```bash
cargo build
```

## Flashing the ESP32

Once the project is built, use one of the following commands to flash the firmware onto your ESP32 board:

- With `cargo espflash`:
  ```bash
  cargo espflash flash
  ```

## Running and Debugging

After flashing, your ESP32 should run the firmware, initializing the OLED and displaying an animated "HELLO WORLD" message. Use logging (via `esp-idf-svc`) or a serial monitor (e.g., `minicom` or `picocom`) to check output messages for debugging.

