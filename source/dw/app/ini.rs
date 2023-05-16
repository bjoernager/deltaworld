// Copyright 2023 Gabriel Jensen.

use crate::dw::app::App;
use crate::dw::VER;

extern crate glfw;

impl App {
	pub fn ini(&mut self) -> i8 {
		eprintln!("DeltaWorld {}.{}.{}",VER.maj,VER.min,VER.pat);

		let mut gfx = self.inigfx();

		while !gfx.win.should_close() {
			gfx.glfw.poll_events();
		}

		return 0x45;
	}
}
