// Copyright 2023 Gabriel Jensen.

mod dw;

use crate::dw::app::App;

use std::process::exit;

fn main() {
	let mut app = App::new();

	exit(app.ini() as i32);
}
