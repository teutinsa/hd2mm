use std::sync::{
	Arc,
	Mutex
};
use eframe::egui::Context;
use crate::app::Hd2mmState;
use super::Page;

pub struct DashboardPage {
	
}

impl DashboardPage {
	pub fn new() -> Self {
		Self {

		}
	}
}

impl Page for DashboardPage {
	fn update(&mut self, data: &Arc<Mutex<Hd2mmState>>) {
		
	}

	fn view(&mut self, data: &Arc<Mutex<Hd2mmState>>, ctx: &Context) {
		
	}
}