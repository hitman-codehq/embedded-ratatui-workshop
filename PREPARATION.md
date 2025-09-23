# Preparation 👨‍🍳

Hello there little rat! 👋

<img src="assets/gusteau.png" width="300">

My name is [Auguste Gusteau](https://disney.fandom.com/wiki/Auguste_Gusteau), and I will be your guide on this cooking journey.

I can feel your excitement! Don't worry, cooking is an art that anyone can master with the right compiler and a bit of terminal magic.

If you ever feel lost, just remember:

![](assets/gusteau.gif)

## Prerequisites

It is very important that you need to get your tools right before entering the kitchen.

> [!IMPORTANT]  
> If you don't have everything set up, I'm afraid we won't have enough time to do hacking during the [workshop](./README.md).

Here is a checklist of what you need:

- [ ] [Rust](#installing-rust)
- [ ] [Rust + ESP32 Toolchain](#installing-esp32-toolchain)
- [ ] [ESP32 Tools](#installing-esp32-tools)
- [ ] A terminal ([Wezterm](https://wezterm.org) is recommended.)
- [ ] A code editor with Rust support (e.g. [VS Code](https://code.visualstudio.com/), [RustRover](https://www.jetbrains.com/rust/), [Neovim](https://neovim.io/), etc.)
- [ ] An USB-C cable that supports data transfer (not just charging)

### Installing Rust

Oh, you are asking what is Rust? You can think of it as our cooking pot. It provides a safe and leak-free environment for us to cook (water & memory leaks).

> [!NOTE]  
> [Rust](https://www.rust-lang.org/) is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It is great for embedded development because of its performance and safety.

To install Rust, you can follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) or use your package manager.

In a nutshell, running this command in your terminal would be enough:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install the essential binaries like:

- `cargo`: Rust's build system and package manager.
- `rustc`: The Rust compiler.

Now, please take your time to check if Rust is installed correctly by running:

```bash
$ rustc --version

rustc 1.90.0 (1159e78c4 2025-09-14)
```

```bash
$ cargo --version

cargo 1.90.0 (840b83a10 2025-07-30)
```

> [!NOTE]  
> Your output may vary depending on the version you installed. It should be all good as long as you don't get any errors.

### Installing ESP32 toolchain

One of the important ingredients that we will drop into our cooking pot is the [**ESP32 T-Display board**](https://lilygo.cc/products/lilygo%C2%AE-ttgo-t-display-1-14-inch-lcd-esp32-control-board):

<img src="assets/t-display.png" width="300">

> [!NOTE]  
> Some general information:
>
> - [Espressif](https://www.espressif.com/) is the company that makes the ESP32 chips and boards. You can find more about them [here](https://www.espressif.com/en/products/socs/esp32).
> - ESP32 chip is based on `Xtensa` architecture, which is different from the `x86_64` architecture that your computer uses. Therefore, we need a special toolchain for compiling Rust code.
> - **T-Display** is one of the development boards that uses ESP32 chip. It has a built-in display, buttons, and other peripherals that we can use for our projects. The display is very useful in our case!

<details>
  <summary>Click here for more specifics.</summary>

| Feature               | Specification                                 |
| --------------------- | --------------------------------------------- |
| MCU                   | ESP32 Xtensa dual-core LX6 microprocessor     |
| CPU                   | Xtensa dual-core LX6 @ up to 240 MHz          |
| RAM                   | 520 KB SRAM (typical for ESP32)               |
| Wireless Connectivity | Wi-Fi 802.11 b/g/n, BL V4.2 + BLE             |
| Serial Chip           | CH9102                                        |
| Optional              | Flash: 4M / 16M                               |
| Onboard Functions     | Buttons: l006 + I007, battery power detection |

</details>

Alright little rat, now that you know what we are going to cook with, let's get the toolchain ready. You will need the [`espup`](https://github.com/esp-rs/espup) tool to install the `Xtensa` toolchain. It can be installed with our friendly neighborhood package manager `cargo`:

```bash
cargo install espup
```

To verify the install:

```bash
$ espup -V
espup 0.15.1
```

And then hit this single-line command to install the toolchain:

```bash
espup install
```

### Installing ESP32 tools

We have raw patatoes, carrots, and onions. But we can't just throw them into the pot without peeling and cutting them first.
In other words, we can build Rust for ESP, but we need to actually put it on the board (i.e. _flash it_).

[`espflash`](https://github.com/esp-rs/espflash) is our friend! Simply installed it by:

```bash
cargo install espflash
```

To verify the install:

```bash
$ espflash -V
espflash 4.0.1
```

> [!IMPORTANT]  
> We won't be calling `espflash` directly. Instead, it is being used as a runner in our [`config.toml`](template/.cargo/config.toml) file and automatically invoked by `cargo` when we run `cargo run`, which is more convenient!

## Testing your setup

Now that you have everything ready, let's start dropping some ingredients into our cooking pot and check out how it tastes!

![](assets/remy-cook.gif)

There is a template project in the `template/` folder that you can use to test your setup.

TODO

Now look at what you did!

![](assets/gusteau-and-remy.png)

## Ending

Now, it's my turn to help you out with what you want to build.

![](assets/gusteau-bon-appetit.png)

_Are you ready to cook?_
