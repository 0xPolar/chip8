use eframe::egui;
use std::sync::{Arc, Mutex};

use super::disassembler;
use super::state::{DebugState, EmulatorSnapshot};

pub struct DebuggerApp {
    state: Arc<Mutex<DebugState>>,
}

impl DebuggerApp {
    fn new(state: Arc<Mutex<DebugState>>) -> Self {
        DebuggerApp { state: state }
    }
}

impl eframe::App for DebuggerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        let (snapshot, paused, breakpoints) = {
            let state = self.state.lock().unwrap();
            (
                state.snapshot.clone(),
                state.paused,
                state.breakpoints.clone(),
            )
        };

        // Left panel: controls, registers, timers
        egui::SidePanel::left("controls_panel")
            .min_width(250.0)
            .show(&ctx, |ui| {
                self.draw_controls(ui, paused);
                ui.separator();
                self.draw_registers(ui, &snapshot);
                ui.separator();
                self.draw_timers(ui, &snapshot);
                ui.separator();
                self.draw_current_instruction(ui, &snapshot);
            });

        // Right panel: stack
        egui::SidePanel::right("stack_panel")
            .min_width(150.0)
            .show(&ctx, |ui| {
                self.draw_stack(ui, &snapshot);
            });

        // Central panel: disassembly + memory
        egui::CentralPanel::default().show(&ctx, |ui| {
            let half = ui.available_height() / 2.0;

            ui.group(|ui| {
                ui.set_min_height(half);
                ui.heading("Disassembly");
                self.draw_disassembly(ui, &snapshot, &breakpoints);
            });

            ui.separator();

            ui.group(|ui| {
                ui.heading("Memory");
                self.draw_memory(ui, &snapshot);
            });
        });

        // Keep refreshing so state updates are visible
        ctx.request_repaint();
    }
}

impl DebuggerApp {
    fn draw_controls(&self, ui: &mut egui::Ui, paused: bool) {
        ui.heading("Controls");

        ui.horizontal(|ui| {
            if ui.button(if paused { "Resume" } else { "Pause" }).clicked() {
                let mut state = self.state.lock().unwrap();
                state.paused = !paused;
            }

            if ui.button("Step").clicked() {
                let mut state = self.state.lock().unwrap();
                state.paused = true;
                state.step_requested = true;
            }
        });

        ui.label(if paused {
            "Status: PAUSED"
        } else {
            "Status: RUNNING"
        });
    }
    fn draw_registers(&self, ui: &mut egui::Ui, snapshot: &EmulatorSnapshot) {
        ui.heading("Registers");

        egui::Grid::new("registers_grid")
            .num_columns(4)
            .spacing([10.0, 4.0])
            .show(ui, |ui| {
                for i in 0..16 {
                    ui.monospace(format!("V{:X}: {:#04X}", i, snapshot.registers[i]));
                    if (i + 1) % 4 == 0 {
                        ui.end_row();
                    }
                }
            });

        ui.add_space(4.0);
        ui.monospace(format!("I:  {:#06X}", snapshot.index));
        ui.monospace(format!("PC: {:#06X}", snapshot.pc));
        ui.monospace(format!("SP: {:#04X}", snapshot.sp));
    }

    fn draw_timers(&self, ui: &mut egui::Ui, snapshot: &EmulatorSnapshot) {
        ui.heading("Timers");
        ui.monospace(format!("Delay: {:#04X} ({})", snapshot.dt, snapshot.dt));
        ui.monospace(format!("Sound: {:#04X} ({})", snapshot.st, snapshot.st));

        if snapshot.is_waiting {
            ui.colored_label(egui::Color32::YELLOW, "Waiting for key press...");
        }
    }

    fn draw_current_instruction(&self, ui: &mut egui::Ui, snapshot: &EmulatorSnapshot) {
        ui.heading("Current Instruction");
        let mnemonic = disassembler::disassemble(snapshot.current_opcode);
        ui.monospace(format!(
            "{:#06X}: {:#06X}",
            snapshot.pc, snapshot.current_opcode
        ));
        ui.monospace(format!("        {}", mnemonic));
    }

    fn draw_disassembly(
        &self,
        ui: &mut egui::Ui,
        snapshot: &EmulatorSnapshot,
        breakpoints: &std::collections::HashSet<u16>,
    ) {
        // Show ~30 instructions centered around PC
        let start_addr = if snapshot.pc >= 20 {
            (snapshot.pc - 20) & 0xFFFE // align to even address
        } else {
            0x200
        };

        let instructions = disassembler::disassemble_region(&snapshot.memory, start_addr, 30);

        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                for (addr, opcode, mnemonic) in &instructions {
                    let is_current = *addr == snapshot.pc;
                    let is_breakpoint = breakpoints.contains(addr);

                    let text = format!(
                        "{} {:#06X}: {:#06X}  {}",
                        if is_breakpoint { "(*)" } else { "   " },
                        addr,
                        opcode,
                        mnemonic
                    );

                    let label = if is_current {
                        egui::RichText::new(&text)
                            .monospace()
                            .background_color(egui::Color32::from_rgb(60, 60, 120))
                            .color(egui::Color32::WHITE)
                    } else if is_breakpoint {
                        egui::RichText::new(&text)
                            .monospace()
                            .color(egui::Color32::RED)
                    } else {
                        egui::RichText::new(&text).monospace()
                    };

                    // Click any instruction row to toggle a breakpoint
                    if ui.add(egui::Label::new(label).sense(egui::Sense::click())).clicked() {
                        let mut state = self.state.lock().unwrap();
                        state.toggle_breakpoint(*addr);
                    }
                }
            });
    }

    fn draw_memory(&self, ui: &mut egui::Ui, snapshot: &EmulatorSnapshot) {
        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                for row_start in (0..4096).step_by(16) {
                    let mut hex_str = format!("{:#06X}: ", row_start);

                    for col in 0..16 {
                        let addr = row_start + col;
                        if addr < snapshot.memory.len() {
                            hex_str.push_str(&format!("{:02X} ", snapshot.memory[addr]));
                        }
                        if col == 7 {
                            hex_str.push(' ');
                        }
                    }

                    ui.monospace(&hex_str);
                }
            });
    }
}
