// Copyright 2023 Gabriel Jensen.

use crate::dw::app::App;
use crate::dw::app::Gfx;

extern crate gl;
extern crate glfw;

use glfw::{Context};
use std::ffi::c_void;

impl App {
	pub fn inigfx(&mut self) -> Gfx {
		eprintln!("initialising glfw");
		let mut glfw = glfw:: init(glfw:: FAIL_ON_ERRORS).unwrap();

		eprintln!("creating window");
		glfw.window_hint(glfw:: WindowHint:: ContextVersion(0x3,0x2));
		glfw.window_hint(glfw:: WindowHint:: OpenGlProfile(glfw:: OpenGlProfileHint:: Core));
		glfw.window_hint(glfw:: WindowHint:: Samples(Some(0x8)));

		
		let (mut win,evt) = glfw.create_window(0x400,0x300,"dw",glfw:: WindowMode:: Windowed).expect("unable to create window");
		win.set_key_polling(true);
		win.make_current();

		eprintln!("initialising opengl");
		gl:: load_with(|nam| glfw.get_proc_address_raw(nam) as *const c_void);

		return Gfx {
			glfw: glfw,
			win: win,
			evt: evt,
		};
	}
}
