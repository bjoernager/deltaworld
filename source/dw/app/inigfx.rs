// Copyright 2023 Gabriel Jensen.

use crate::dw::VER;
use crate::dw::app::{App, Gfx};

extern crate gl;
extern crate glfw;

use gl::{GetUniformLocation, load_with};
use gl::types::{GLchar, GLint};
use glfw::{Context, FAIL_ON_ERRORS, init, SwapInterval, WindowHint};
use std::ffi::c_void;

impl App {
	pub fn inigfx(&mut self) -> Gfx {
		eprintln!("initialising glfw");
		let mut glfw = init(FAIL_ON_ERRORS).unwrap();

		eprintln!("creating window");
		glfw.window_hint(WindowHint::ContextVersion(0x3, 0x2));
		glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
		glfw.window_hint(WindowHint::Samples(Some(0x8)));

		let (mut win, evt) = glfw.create_window(0x400, 0x300, format!("DeltaWorld {}.{}.{}", VER.maj, VER.min, VER.pat).as_str(), glfw::WindowMode::Windowed).expect("unable to create window");
		win.set_key_polling(true);

		eprintln!("initialising opengl");
		win.make_current();
		load_with(|nam| glfw.get_proc_address_raw(nam) as *const c_void);

		glfw.set_swap_interval(SwapInterval::Sync(0x1));

		let shdprg = self.getshdprg();

		let uni: GLint = unsafe {
			GetUniformLocation(shdprg, b"scl\x00".as_ptr() as *const GLchar)
		};

		return Gfx {
			evt:    evt,
			glfw:   glfw,
			shdprg: shdprg,
			uni:    uni,
			win:    win,
		};
	}
}
