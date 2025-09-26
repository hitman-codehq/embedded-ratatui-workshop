# Ratatui Mousefood Template

This is a template for you to start your own Ratatui project for embedded devices.
It includes all the necessary configuration files and a minimal example to get you started.

## Running

> [!WARNING]
> Make sure you have completed the prerequisites in [preparation steps](PREPARATION.md).

To run the example application, connect your ESP32 device to your computer and execute:

```bash
cargo run --release
```

This command will build the project, flash it to the connected device, and open a serial monitor to view the output.

## Project Structure

```
.
├── build.rs             -> Build script to set up ESP-IDF environment
├── .cargo
│   └── config.toml      -> Cargo configuration for ESP32
├── Cargo.toml           -> Dependencies and project metadata
├── rust-toolchain.toml  -> Specifies Rust toolchain for ESP32
├── sdkconfig.defaults   -> ESP-IDF configuration (such as stack size)
└── src
    ├── button.rs        -> Button handling module
    ├── lib.rs           -> Module definitions
    ├── main.rs          -> Main application
    └── setup.rs         -> Setup and initialization code
```

`main.rs` is the file that you will be mostly interacting with and building your application. See the [design choices](#design-choices) section for more details about the other source files.

### `.cargo/config.toml`

This file is important as it configures Cargo to use [`espflash`](https://github.com/esp-rs/espflash) to flash the firmware to the device:

```toml
[target.xtensa-esp32-espidf]
linker = "ldproxy"
runner = "espflash flash --monitor"
rustflags = [ "--cfg",  "espidf_time64"]
```

Alternatively, you can use [`web-flash`](https://github.com/esp-rs/esp-web-flash-server) to flash the firmware over your web browser:

```toml
[target.xtensa-esp32-espidf]
linker = "ldproxy"
runner = "web-flash --chip=esp32 "
rustflags = [ "--cfg",  "espidf_time64"]
```

## Design Choices

We have an `App` trait that controls the application lifecycle. The user needs to implement this trait to define their application logic.

It provides three main methods:

- `draw(frame: Frame)`: This method is called in every iteration of the main loop to render the UI.
- `handle_press(button: Button)`: This method is called whenever a button press event is detected.
- `run()`: This method starts the application. It has a default implementation that sets up the main loop and handles events.

There is a minimal implementation of the `App` trait in `main.rs` that you can use as a starting point.
