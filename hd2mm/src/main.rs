#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_min_inner_size([800.0, 600.0]),
		centered: true,
		..Default::default()
	};
	eframe::run_native("Helldivers 2 Mod Manager", options, Box::new(|_| Ok(Box::<Hd2mmApp>::default())))
}

enum AppState {
	Loading,
	FirstTimeSetup,
	Dashboard
}

struct Hd2mmApp {
	state: AppState,
	worker: Option<std::thread::Thread>
}

impl Hd2mmApp {
	
}

impl Default for Hd2mmApp {
	fn default() -> Self {
		Self {
			state: AppState::Loading,
			worker: None
		}
	}
}

impl eframe::App for Hd2mmApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		match &mut self.state {
			AppState::Loading => {
				egui::CentralPanel::default().show(ctx, |ui| {
					ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
						ui.add(egui::Spinner::new().size(50.0));
					});
				});
			}
			AppState::Dashboard => {

			}
			_ => {
				egui::CentralPanel::default().show(ctx, |ui| {
					ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui| {
						ui.label("!!!UNIMPLEMENTED!!!");
					});
				});
			}
		}
	}

	fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
		
	}
}