use std::{
	sync::{
		Arc,
		Mutex
	},
	thread::JoinHandle
};
use eframe::egui;
use crate::app::Hd2mmState;
use super::{
	dashboard::DashboardPage,
	Page
};

pub struct LoadingPage {
	message: Arc<Mutex<String>>,
	handle: Option<JoinHandle<()>>
}

impl LoadingPage {
	pub fn new() -> Self {
		Self {
			message: Arc::new(Mutex::new("Loading...".to_string())),
			handle: None
		}
	}
}

impl Page for LoadingPage {
	fn update(&mut self, data: &Arc<Mutex<Hd2mmState>>) {
		if self.handle.is_none() {
			let message = self.message.clone();
			let data = data.clone();
			self.handle = Some(std::thread::spawn(move || {
				*message.lock().unwrap() = "Looking for settings...".to_string();


				
				*message.lock().unwrap() = "Done.".to_string();
				data.lock().unwrap().next_page = Some(Box::new(DashboardPage::new()));
			}));
		}
	}

	fn view(&mut self, data: &Arc<Mutex<Hd2mmState>>, ctx: &eframe::egui::Context) {
		egui::CentralPanel::default().show(ctx, |ui| {
			egui::Frame::none()
			.outer_margin(200.0)
			.show(ui, |ui| {
				ui.vertical_centered_justified(|ui| {
					ui.label(
						egui::RichText::new("Loading")
						.size(32.0)
					);
					ui.add(
						egui::Spinner::new()
						.size(50.0)
					);
					ui.label(&*self.message.lock().unwrap());
				});
			});
		});
	}
}