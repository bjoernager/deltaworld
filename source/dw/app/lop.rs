// Copyright 2023 Gabriel Jensen.

use crate::dw::app::{App,GOTINT};
use crate::dw::app::Gfx;

//extern crate gl;
extern crate glfw;

use std::sync::atomic::Ordering;

impl App {
	pub fn lop(&mut self,gfx: &mut Gfx) -> i8 {
		eprintln!("entering main loop");

		while !gfx.win.should_close() {
			unsafe {
				if GOTINT.load(Ordering::Relaxed) {
					eprintln!("got interrupt");
					gfx.win.set_should_close(true);
				}
			}

			gfx.glfw.poll_events();
		}

		return -0x45;
	}
}
