pub mod pages;

use std::sync::{
	Arc,
	Mutex
};
use eframe::{
	egui::{
		self,
		Context,
		ViewportBuilder
	},
	App,
	CreationContext,
	Frame,
	NativeOptions
};
use hd2mm_lib::ModManager;
use pages::{
	loading::LoadingPage,
	Page
};

pub struct Hd2mmState {
	pub next_page: Option<Box<dyn Page>>,
	pub manager: Option<ModManager>
}

impl Default for Hd2mmState {
	fn default() -> Self {
		Self {
			next_page: Some(Box::new(LoadingPage::new())),
			manager: None
		}
	}
}

pub struct Hd2mmApp {
	data: Arc<Mutex<Hd2mmState>>,
	page: Option<Box<dyn Page>>
}

impl Hd2mmApp {
	fn new(_cc: &CreationContext<'_>) -> Self {
		Self {
			data: Arc::default(),
			page: None
		}
	}

	pub fn run() -> eframe::Result {
		let options = NativeOptions {
			centered: true,
			vsync: true,
			viewport: ViewportBuilder::default()
				.with_min_inner_size([800.0, 600.0]),
			..Default::default()
		};
		eframe::run_native(
			"Helldivers 2 Mod Manager",
			options,
			Box::new(|cc| Ok(Box::new(Hd2mmApp::new(cc))))
		)
	}
}

impl App for Hd2mmApp {
	fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
		let has_next = self.data.lock().unwrap().next_page.is_some();
		if has_next {
			let next = std::mem::replace(&mut self.data.lock().unwrap().next_page, None);
			self.page = next;
		}
		if let Some(page) = &mut self.page {
			page.update(&self.data);
			page.view(&self.data, ctx);
		} else {
			egui::CentralPanel::default().show(ctx, |ui| {
				ui.centered_and_justified(|ui| {
					ui.label(
						egui::RichText::new("Page error!")
							.color(egui::Color32::RED)
							.size(48.0)
					);
				})
			});
		}
	}
}