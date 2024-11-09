#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::Hd2mmApp;

pub mod app;

fn main() -> eframe::Result {
	Hd2mmApp::run()
}