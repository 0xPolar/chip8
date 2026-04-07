use imgui;

use super::disassembler;
use super::state::{DebugState, EmulatorSnapshot};

use std::collections::HashSet;

pub fn draw_all(ui: &imgui::Ui, state: &mut DebugState, texture_id: imgui::TextureId) {
    let [win_w, win_h] = ui.io().display_size;

    // Column widths
    let left_w = (win_w * 0.22).max(200.0);
    let right_w = (win_w * 0.18).max(160.0);
    let mid_w = win_w - left_w - right_w;

    // Left column heights
    let reg_h = (win_h * 0.36).max(180.0);
    let timer_h = (win_h * 0.14).max(70.0);
    let instr_h = (win_h * 0.12).max(60.0);
    let mem_h = win_h - reg_h - timer_h - instr_h;

    // Right column heights
    let stack_h = (win_h * 0.5).max(150.0);
    let ctrl_h = win_h - stack_h;

    // Middle row heights
    let display_h = (win_h * 0.42).max(200.0);
    let disasm_h = win_h - display_h;

    draw_registers(ui, &state.snapshot, [0.0, 0.0], [left_w, reg_h]);
    draw_timers(ui, &state.snapshot, [0.0, reg_h], [left_w, timer_h]);
    draw_current_instruction(
        ui,
        &state.snapshot,
        [0.0, reg_h + timer_h],
        [left_w, instr_h],
    );
    draw_memory(
        ui,
        &state.snapshot,
        [0.0, reg_h + timer_h + instr_h],
        [left_w, mem_h],
    );

    draw_game_display(ui, texture_id, [left_w, 0.0], [mid_w, display_h]);
    draw_disassembly(
        ui,
        &state.snapshot,
        &mut state.breakpoints,
        [left_w, display_h],
        [mid_w, disasm_h],
    );

    draw_stack(
        ui,
        &state.snapshot,
        [left_w + mid_w, 0.0],
        [right_w, stack_h],
    );
    draw_controls(ui, state, [left_w + mid_w, stack_h], [right_w, ctrl_h]);
}

fn draw_game_display(ui: &imgui::Ui, chip8_tex: imgui::TextureId, pos: [f32; 2], size: [f32; 2]) {
    ui.window("CHIP-8 Display")
        .position(pos, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .build(|| {
            let avail = ui.content_region_avail();
            let aspect = 2.0; // 64:32 = 2:1
            let img_w = avail[0];
            let img_h = img_w / aspect;
            let (img_w, img_h) = if img_h > avail[1] {
                (avail[1] * aspect, avail[1])
            } else {
                (img_w, img_h)
            };
            imgui::Image::new(chip8_tex, [img_w, img_h]).build(ui);
        });
}

fn draw_registers(ui: &imgui::Ui, snap: &EmulatorSnapshot, pos: [f32; 2], size: [f32; 2]) {
    ui.window("Registers")
        .position(pos, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .build(|| {
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

fn draw_timers(ui: &imgui::Ui, snap: &EmulatorSnapshot, pos: [f32; 2], size: [f32; 2]) {
    ui.window("Timers")
        .position(pos, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .build(|| {
            ui.text(format!("Delay: {:#04X} ({})", snap.DT, snap.DT));
            ui.text(format!("Sound: {:#04X} ({})", snap.ST, snap.ST));
            if snap.waiting {
                ui.text_colored([1.0, 1.0, 0.0, 1.0], "Waiting for key press...");
            }
        });
}

fn draw_current_instruction(
    ui: &imgui::Ui,
    snap: &EmulatorSnapshot,
    pos: [f32; 2],
    size: [f32; 2],
) {
    ui.window("Current Instruction")
        .position(pos, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .build(|| {
            let instruction = disassembler::disassemble(snap.current_opcode);
            ui.text(format!("{:#06X}: {:#06X}", snap.PC, snap.current_opcode));
            ui.text(format!("        {}", instruction));
        });
}

fn draw_controls(ui: &imgui::Ui, state: &mut DebugState, pos: [f32; 2], size: [f32; 2]) {
    ui.window("Controls")
        .position(pos, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
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
            if ui.button("Reset") {
                state.reset_requested = true;
                state.paused = true;
            }
            ui.text(if state.paused {
                "Status: PAUSED"
            } else {
                "Status: RUNNING"
            });
        });
}

fn draw_stack(ui: &imgui::Ui, snap: &EmulatorSnapshot, pos: [f32; 2], size: [f32; 2]) {
    ui.window("Stack")
        .position(pos, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
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

fn draw_disassembly(
    ui: &imgui::Ui,
    snap: &EmulatorSnapshot,
    breakpoints: &mut HashSet<u16>,
    pos: [f32; 2],
    size: [f32; 2],
) {
    ui.window("Disassembly")
        .position(pos, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
        .build(|| {
            let start_addr = if snap.PC >= 20 {
                (snap.PC - 20) & 0xFFFE
            } else {
                0x200
            };

            let instructions = disassembler::disassemble_region(&snap.memory, start_addr, 30);

            ui.child_window("disasm_scroll").size([0.0, 0.0]).build(|| {
                for (addr, opcode, instruction) in &instructions {
                    let is_current = *addr == snap.PC;
                    let is_breakpoint = breakpoints.contains(addr);

                    let prefix = if is_breakpoint { "(*)" } else { "   " };
                    let text =
                        format!("{} {:#06X}: {:#06X}  {}", prefix, addr, opcode, instruction);

                    if is_current {
                        let _color =
                            ui.push_style_color(imgui::StyleColor::Header, [0.24, 0.24, 0.47, 1.0]);
                        if ui.selectable_config(&text).selected(true).build() {
                            toggle_breakpoint(breakpoints, *addr);
                        }
                    } else if is_breakpoint {
                        let _color =
                            ui.push_style_color(imgui::StyleColor::Text, [1.0, 0.3, 0.3, 1.0]);
                        if ui.selectable(&text) {
                            toggle_breakpoint(breakpoints, *addr);
                        }
                    } else {
                        if ui.selectable(&text) {
                            toggle_breakpoint(breakpoints, *addr);
                        }
                    }
                }
            });
        });
}

fn draw_memory(ui: &imgui::Ui, snap: &EmulatorSnapshot, pos: [f32; 2], size: [f32; 2]) {
    ui.window("Memory")
        .position(pos, imgui::Condition::Always)
        .size(size, imgui::Condition::Always)
        .movable(false)
        .resizable(false)
        .collapsible(false)
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
