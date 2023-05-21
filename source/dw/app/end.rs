// Copyright 2023 Gabriel Jensen.

use crate::dw::app::{App, Gfx};

extern crate gl;

use gl::{DeleteProgram};

impl App {
	pub fn end(&mut self,gfx: &mut Gfx) {
		eprintln!("ending");

		unsafe {
			DeleteProgram(gfx.shdprg);
		}

		// GLFW is automatically "destroyed".
	}
}
