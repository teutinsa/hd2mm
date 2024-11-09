pub mod dashboard;
pub mod loading;

use std::sync::{
	Arc,
	Mutex
};
use eframe::egui::Context;
use super::Hd2mmState;

pub trait Page: Send {
	fn update(&mut self, data: &Arc<Mutex<Hd2mmState>>);

	fn view(&mut self, data: &Arc<Mutex<Hd2mmState>>, ctx: &Context);
}