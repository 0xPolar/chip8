use imgui;

use super::disassembler;
use super::state::{DebugState, EmulatorSnapshot};

use std::collections::HashSet;

pub fn draw_all(ui: &imgui::Ui, state: &mut DebugState, texture_id: imgui::TextureId) {
    draw_game_display(ui, texture_id);
    draw_registers(ui, &state.snapshot);
    draw_timers(ui, &state.snapshot);
    draw_current_instruction(ui, &state.snapshot);
    draw_stack(ui, &state.snapshot);
    draw_controls(ui, state);
    draw_disassembly(ui, &state.snapshot, &mut state.breakpoints);
    draw_memory(ui, &state.snapshot);
}

fn draw_game_display(ui: &imgui::Ui, chip8_tex: imgui::TextureId) {
    ui.window("CHIP-8 Display")
        .position([250.0, 0.0], imgui::Condition::FirstUseEver)
        .size([528.0, 288.0], imgui::Condition::FirstUseEver)
        .build(|| {
            imgui::Image::new(chip8_tex, [512.0, 256.0]).build(&ui);
        });
}

fn draw_registers(ui: &imgui::Ui, snap: &EmulatorSnapshot) {
    ui.window("Registers")
        .position([0.0, 0.0], imgui::Condition::FirstUseEver)
        .size([250.0, 250.0], imgui::Condition::FirstUseEver)
        .build(|| {
            // 4-column grid for V0-VF
            ui.columns(4, "reg_cols", false);
            for i in 0..16 {
                ui.text(format!("V{:X}: {:02X}", i, snap.regs[i]));
                ui.next_column();
            }
            ui.columns(1, "end", false);

            ui.separator();
            ui.text(format!("I:  {:#06X}", snap.index));
            ui.text(format!("PC: {:#06X}", snap.PC));
            ui.text(format!("SP: {:#04X}", snap.SP));
        });
}

fn draw_timers(ui: &imgui::Ui, snap: &EmulatorSnapshot) {
    ui.window("Timers")
        .position([0.0, 250.0], imgui::Condition::FirstUseEver)
        .size([250.0, 100.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("Delay: {:#04X} ({})", snap.DT, snap.DT));
            ui.text(format!("Sound: {:#04X} ({})", snap.ST, snap.ST));
            if snap.waiting {
                ui.text_colored([1.0, 1.0, 0.0, 1.0], "Waiting for key press...");
            }
        });
}

fn draw_current_instruction(ui: &imgui::Ui, snap: &EmulatorSnapshot) {
    ui.window("Current Instruction")
        .position([0.0, 350.0], imgui::Condition::FirstUseEver)
        .size([250.0, 80.0], imgui::Condition::FirstUseEver)
        .build(|| {
            let mnemonic = disassembler::disassemble(snap.current_opcode);
            ui.text(format!("{:#06X}: {:#06X}", snap.PC, snap.current_opcode));
            ui.text(format!("        {}", mnemonic));
        });
}

fn draw_controls(ui: &imgui::Ui, state: &mut DebugState) {
    ui.window("Controls")
        .position([790.0, 280.0], imgui::Condition::FirstUseEver)
        .size([200.0, 200.0], imgui::Condition::FirstUseEver)
        .build(|| {
            if ui.button(if state.paused { "Resume" } else { "Pause" }) {
                state.paused = !state.paused;
            }
            ui.same_line();
            if ui.button("Step") {
                state.paused = true;
                state.step_requested = true;
            }

            ui.separator();
            ui.text(if state.paused {
                "Status: PAUSED"
            } else {
                "Status: RUNNING"
            });
        });
}

fn draw_stack(ui: &imgui::Ui, snap: &EmulatorSnapshot) {
    ui.window("Stack")
        .position([790.0, 0.0], imgui::Condition::FirstUseEver)
        .size([200.0, 280.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.text(format!("SP: {}", snap.SP));
            ui.separator();
            if snap.SP == 0 {
                ui.text("(empty)");
            } else {
                for i in (0..snap.SP as usize).rev() {
                    let marker = if i == (snap.SP as usize) - 1 {
                        ">"
                    } else {
                        " "
                    };
                    ui.text(format!("{} [{:2}] {:#06X}", marker, i, snap.stack[i]));
                }
            }
        });
}

fn draw_disassembly(ui: &imgui::Ui, snap: &EmulatorSnapshot, breakpoints: &mut HashSet<u16>) {
    ui.window("Disassembly")
        .position([250.0, 288.0], imgui::Condition::FirstUseEver)
        .size([540.0, 400.0], imgui::Condition::FirstUseEver)
        .build(|| {
            let start_addr = if snap.PC >= 20 {
                (snap.PC - 20) & 0xFFFE
            } else {
                0x200
            };

            let instructions = disassembler::disassemble_region(&snap.memory, start_addr, 30);

            ui.child_window("disasm_scroll").size([0.0, 0.0]).build(|| {
                for (addr, opcode, mnemonic) in &instructions {
                    let is_current = *addr == snap.PC;
                    let is_breakpoint = breakpoints.contains(addr);

                    let prefix = if is_breakpoint { "(*)" } else { "   " };
                    let text = format!("{} {:#06X}: {:#06X}  {}", prefix, addr, opcode, mnemonic);

                    // Highlight current instruction
                    if is_current {
                        // Push a style color for the selected highlight
                        let _color =
                            ui.push_style_color(imgui::StyleColor::Header, [0.24, 0.24, 0.47, 1.0]);
                        // selectable_config lets you set .selected(true)
                        // build() takes NO arguments (unlike Image::build)
                        if ui.selectable_config(&text).selected(true).build() {
                            toggle_breakpoint(breakpoints, *addr);
                        }
                    } else if is_breakpoint {
                        ui.text_colored([1.0, 0.3, 0.3, 1.0], &text);
                    } else {
                        // Simple selectable -- returns true when clicked
                        if ui.selectable(&text) {
                            toggle_breakpoint(breakpoints, *addr);
                        }
                    }
                }
            });
        });
}

fn draw_memory(ui: &imgui::Ui, snap: &EmulatorSnapshot) {
    ui.window("Memory")
        .position([0.0, 430.0], imgui::Condition::FirstUseEver)
        .size([250.0, 270.0], imgui::Condition::FirstUseEver)
        .build(|| {
            ui.child_window("memory_scroll").size([0.0, 0.0]).build(|| {
                for row_start in (0..4096).step_by(16) {
                    let mut hex_str = format!("{:#06X}: ", row_start);
                    for col in 0..16 {
                        let addr = row_start + col;
                        if addr < snap.memory.len() {
                            hex_str.push_str(&format!("{:02X} ", snap.memory[addr]));
                        }
                        if col == 7 {
                            hex_str.push(' ');
                        }
                    }
                    ui.text(&hex_str);
                }
            });
        });
}

fn toggle_breakpoint(breakpoints: &mut HashSet<u16>, addr: u16) {
    if !breakpoints.remove(&addr) {
        breakpoints.insert(addr);
    }
}
