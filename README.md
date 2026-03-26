# CHIP-8 Emulator

A CHIP-8 emulator written in Rust using raylib for graphics and input handling.

## Why I Built This

I built this project to learn Rust and explore how emulators work at a low level. CHIP-8 is a great starting point for emulation — it has a simple architecture but still covers all the fundamentals: fetch-decode-execute cycles, memory-mapped I/O, timers, sprite rendering, and input handling.

<!-- TODO: Add a screenshot of the emulator running a ROM -->
<!-- ![Screenshot](screenshot.png) -->

## Features

- Complete CHIP-8 instruction set (35 opcodes)
- 64x32 pixel display rendered at 10x scale (640x320 window)
- 16-key hexadecimal keypad mapped to QWERTY keyboard
- 60Hz delay and sound timers
- XOR-based sprite drawing with collision detection
- Accurate cycle timing scaled to ~500Hz CPU speed

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (edition 2024)
- On Linux, install raylib dependencies:
  ```bash
  sudo apt install libx11-dev libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev libgl-dev cmake
  ```

### Build

```bash
cargo build --release
```

### Run

Pass a ROM file as a command-line argument:

```bash
cargo run --release -- path/to/rom.ch8
```

## Keyboard Mapping

The CHIP-8 hex keypad is mapped to your keyboard as follows:

| CHIP-8 | Keyboard |
|--------|----------|
| `1` `2` `3` `C` | `1` `2` `3` `4` |
| `4` `5` `6` `D` | `Q` `W` `E` `R` |
| `7` `8` `9` `E` | `A` `S` `D` `F` |
| `A` `0` `B` `F` | `Z` `X` `C` `V` |

## Running Tests

```bash
cargo test
```

The test suite covers CPU instruction decoding and execution, display rendering and sprite collision, memory layout and ROM loading.

## Architecture

```
src/
├── main.rs              # Entry point, main loop
└── internal/
    ├── chip8.rs         # Emulator core — memory, ROM loading, font data
    ├── cpu.rs           # CPU registers, instruction fetch/decode/execute
    ├── display.rs       # 64x32 pixel buffer, sprite drawing, collision
    ├── graphics.rs      # Raylib window, rendering, input polling
    ├── keypad.rs        # 16-key input state and key mapping
    ├── font.rs          # Built-in 4x5 font sprites
    └── audio.rs         # Sound (stub)
```

## License

MIT
