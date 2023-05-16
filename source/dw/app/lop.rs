// Copyright 2023 Gabriel Jensen.

use crate::dw::app::App;
use crate::dw::app::Gfx;

//extern crate gl;
extern crate glfw;

use glfw::{Context};

impl App {
	pub fn lop(&mut self,gfx: &mut Gfx) -> i8 {
		eprintln!("entering main loop");

		while !gfx.win.should_close() {
			gfx.glfw.poll_events();
		}

		return -0x45;
	}
}
