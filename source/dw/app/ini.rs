// Copyright 2023 Gabriel Jensen.

use crate::dw::app::App;
use crate::dw::VER;

extern crate glfw;

impl App {
	pub fn ini(&mut self) -> i8 {
		println!("\x1B[0m\x1B[1mDeltaWorld\x1B[0m {}.{}.{} \u{2014} Copyright 2023 \x1B[0m\x1B[1mGabriel Jensen\x1B[0m.\n",VER.maj,VER.min,VER.pat);

		let mut gfx = self.inigfx();

		let cod = self.lop(&mut gfx);

		println!("goodbye");
		eprintln!("exiting with code {}",cod);
		return cod;
	}
}
