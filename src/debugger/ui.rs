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
}
