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
        let (snapshot, paused, brakepoints) = {
            let state = self.state.lock().unwrap();
            (
                state.snapshot.clone(),
                state.paused,
                state.breakpoints.clone(),
            )
        };

        egui::SidePanel::left("controls_panel")
            .min_width(250.0)
            .show(ctx, |ui| {
                self.draw_contorls(ui, paused);
                ui.separator();
                self.draw_registers(ui, &snapshot);
                ui.separator();
                self.draw_timers(ui, &snapshot);
                ui.separator();
                self.draw_current_indtruction(ui, &snapshot);
            });

        egui::SidePanel::right("stack_panel")
            .min_width(150.0)
            .show(ctx, |ui| {
                self.draw_stack(ui, &snapshot);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let half = ui.available_height() / 2.0;

            ui.group(|ui| {
                ui.set_min_height(half);
                ui.heading("Disassembely");
                self.draw_disassembly(ui, &snapshot, &brakepoints);
            });

            ui.separator();

            ui.group(|ui| {
                ui.heading("Memory");
                self.draw_memory(ui, &snapshot);
            });
        });

        ctx.request_repaint();
    }
}
