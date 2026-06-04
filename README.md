# CHIP-8 Emulator

A CHIP-8 emulator and debugger written in Rust. It uses SDL2 and OpenGL for the
window and display, ImGui for the debugger interface, and rodio for audio.

## Features

- CHIP-8 CPU with the standard 35-opcode instruction set
- 4 KB memory, built-in CHIP-8 font data, stack, keypad, and timers
- 64x32 monochrome display with XOR sprite drawing and collision detection
- Resizable SDL2/OpenGL window with nearest-neighbor display scaling
- 440 Hz tone while the CHIP-8 sound timer is active
- CPU execution at approximately 500 Hz and timers at 60 Hz
- Integrated debugger with:
  - Register, timer, stack, and memory views
  - Current-instruction and nearby-disassembly views
  - Clickable breakpoints
  - Pause, resume, single-step, and reset controls

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) with support for Rust edition 2024
- CMake and native OpenGL/audio development libraries

On Ubuntu or Debian:

```bash
sudo apt install cmake pkg-config libgl-dev libasound2-dev
```

SDL2 is built from source by the `sdl2` crate's `bundled` feature.

### Build

```bash
cargo build --release
```

### Run

Pass a CHIP-8 ROM path as the first command-line argument:

```bash
cargo run --release -- roms/pong.ch8
```

The repository includes `ibm.ch8`, `pong.ch8`, and `RPS.ch8` in the `roms/`
directory.

## Controls

The CHIP-8 hexadecimal keypad is mapped to a QWERTY keyboard:

| CHIP-8 | Keyboard |
|--------|----------|
| `1` `2` `3` `C` | `1` `2` `3` `4` |
| `4` `5` `6` `D` | `Q` `W` `E` `R` |
| `7` `8` `9` `E` | `A` `S` `D` `F` |
| `A` `0` `B` `F` | `Z` `X` `C` `V` |

The debugger controls are available in the right-hand **Controls** panel. Click
an instruction in the disassembly view to toggle a breakpoint.

## Testing

```bash
cargo test
```

The unit tests cover CPU instructions, timers, keypad behavior, ROM and font
loading, display rendering and collision detection, and disassembly.

## Project Structure

```text
.
|-- .github/workflows/ci.yml   # Windows/Linux test, build, and release workflow
|-- roms/                      # Example CHIP-8 ROMs
|-- src/
|   |-- debugger/
|   |   |-- disassembler.rs    # Opcode disassembly
|   |   |-- state.rs           # Emulator snapshots and debugger state
|   |   |-- ui.rs              # ImGui debugger panels and controls
|   |   `-- mod.rs
|   |-- internal/
|   |   |-- audio.rs           # Rodio sound-timer tone
|   |   |-- chip8.rs           # Emulator core, memory, and ROM loading
|   |   |-- cpu.rs             # Fetch, decode, execute, registers, and timers
|   |   |-- display.rs         # 64x32 display buffer and sprite drawing
|   |   |-- font.rs            # Built-in CHIP-8 font sprites
|   |   |-- graphics.rs        # SDL2, OpenGL, ImGui, and keyboard input
|   |   |-- keypad.rs          # CHIP-8 keypad state
|   |   `-- mod.rs
|   `-- main.rs                # Application loop and debugger integration
|-- Cargo.toml                 # Package metadata and dependencies
`-- Cargo.lock                 # Locked dependency versions
```

## License

MIT
