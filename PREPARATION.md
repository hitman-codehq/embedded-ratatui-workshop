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

---

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

- `cargo`: Rust's build system and package manager (brings you fresh ingredients from crates.io).
- `rustc`: The Rust compiler (makes sure everything is cooked perfectly).

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

---

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

There are two main Rust frameworks (i.e. hardware abstraction layers - HALs) for ESP32:

| **esp-hal**                     | **esp-idf-hal**                |
| ------------------------------- | ------------------------------ |
| Bare metal (`#![no_std]`)       | With standard library support! |
| Development funded by Espressif | Community effort               |
|                                 | Requires a custom toolchain    |

> [!IMPORTANT]  
> `esp-hal` is like cooking without any utensils, while `esp-idf-hal` is like cooking with a full set of utensils and appliances. Both have their own advantages and disadvantages.
>
> We will be using the [`esp-idf-hal`](https://github.com/esp-rs/esp-idf-hal) framework for simplicity and ease of use.

<details>
  <summary>Click here for more specifics.</summary>

ESP-IDF works with FreeRTOS, which is a real-time operating system that provides multitasking and other features. This makes it easier to build complex applications that require concurrency and real-time performance.

On the Rust side, it maps to the standard library, which means you can use familiar Rust constructs like `Vec`, `String`, and `Box`. This makes it easier to write and maintain code, especially for those who are already familiar with Rust.

</details>

Alright little rat, now that you know what we are going to cook with, let's get the ESP-IDF toolchain ready. You will need the [`espup`](https://github.com/esp-rs/espup) tool to install the `Xtensa` toolchain. It can be installed with our friendly neighborhood package manager `cargo`:

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

In case you are wondering, this command will install:

- [Espressif Rust fork](https://github.com/esp-rs/rust) with support for Espressif targets
- `stable` toolchain with support for RISC-V targets
- [LLVM fork](https://github.com/espressif/llvm-project) with support for Xtensa targets
- [GCC toolchain](https://github.com/espressif/crosstool-NG/) that links the final binary

and does not contain any rat poison, don't worry.

If everything goes right, you will have `xtensa-esp32-espidf` toolchain for ESP-IDF framework installed and see a message like this:

> [info]: Installation successfully completed!
> To get started, you need to set up some environment variables by running: '. ~/export-esp.sh'
> This step must be done every time you open a new terminal.
> See other methods for setting the environment in https://esp-rs.github.io/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables

This step is like chopping the onions, measuring the flour, and lining up your spices before turning on the stove.
Otherwise, chaos in the kitchen!

Simply follow the steps [here](https://docs.espressif.com/projects/rust/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables).

---

### Installing ESP32 tools

We have got potatoes, carrots, and onions (the holy trinity of any good ratatouille). But we can't just throw them into the pot without peeling and cutting them first.
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

---

## Testing your setup

Now that you have everything ready, let's start dropping some ingredients into our cooking pot and check out how it tastes!

![](assets/remy-cook.gif)

There is a template project in the [template](template/) folder that you can use to test your setup. See the [documentation there](template/README.md) for more details about specific files.

You can build and flash that project to your ESP32 board by running:

```bash
cd template/

cargo run --release
```

If you want to see something more impressive, try running one of these applications in the [apps](apps/) folder. For example:

```bash
git submodule update --init --recursive
cd apps/mousefood-esp32-demo
cargo run
```

Now look at what you did!

<p align="center">

<img src="assets/gusteau-and-remy.png" width="700">

<img src="assets/mousefood-demo.gif" width="600">

</p>

---

## Ending

Congratulations, now you are a little chef rat! 👨‍🍳

But the world of cooking is vast and there is much more to explore. Now, lemme teach you some recipes.

<img src="assets/gusteau-bon-appetit.png" width="700">

_"Remember, anyone can cook — but only the fearless can be great."_

---

## Troubleshooting

Oh no!

![](assets/remy-walk.gif)

If you encounter any problems and if you need help, please let the chefs know by [creating a new issue](https://github.com/orhun/pocket-sized-tui-workshop/issues/new).

### Windows-specific issues

If you are on Windows without WSL2, things will be much harder. We recommend to install WSL2 and run everything from there.

### MacOS-specific issues

You might hit permission issues with the USB serial.

See [this link](https://docs.esp-rs.org/std-training/02_1_hardware.html) for checking the hardware.

### Linux-specific issues

There can't be any issues. Linux is just perfect.
