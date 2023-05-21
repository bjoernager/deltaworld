// Copyright 2023 Gabriel Jensen.

use crate::dw::app::{App, GOTINT};
use crate::dw::app::Gfx;

extern crate gl;
extern crate glfw;

use gl::{ARRAY_BUFFER, BindBuffer, BufferData, BindVertexArray, BufferSubData, Clear, ClearColor, COLOR_BUFFER_BIT, DrawArrays, EnableVertexAttribArray, FALSE, FLOAT, GenBuffers, GenVertexArrays, STREAM_DRAW, TRIANGLES, UseProgram, VertexAttribPointer};
use gl::types::{GLfloat, GLsizeiptr, GLuint};
use glfw::Context;
use std::ffi::c_void;
use std::mem::{size_of, size_of_val};
use std::ptr::{addr_of, null};
use std::sync::atomic::Ordering;

impl App {
	pub fn lop(&mut self, gfx: &mut Gfx) -> i8 {
		eprintln!("entering main loop");

		let vtx: [GLfloat; 0x9] = [
			-1.0, 1.0, 0.0,
			 1.0, 1.0, 0.0,
			 0.0, 0.0, 0.0,
		];

		let mut vao: GLuint = 0x0;
		let mut vbo: GLuint = 0x0;

		unsafe {
			GenVertexArrays(0x1,&mut vao);
			GenBuffers(0x1,&mut vbo);

			BindVertexArray(vao);

			BindBuffer(ARRAY_BUFFER,vbo);
			BufferData(ARRAY_BUFFER,size_of_val(& vtx) as GLsizeiptr, addr_of!(vtx) as *const c_void, STREAM_DRAW);

			VertexAttribPointer(0x0, 0x3, FLOAT, FALSE, (0x3*size_of::<GLfloat>()) as i32, null::<c_void>());
			EnableVertexAttribArray(0x0);
		}

		while !gfx.win.should_close() {
			unsafe {
				if GOTINT.load(Ordering::Relaxed) {
					eprintln!("got interrupt");
					gfx.win.set_should_close(true);
				}
			}

			gfx.glfw.poll_events();

			unsafe {
				ClearColor(0.107, 0.690, 0.939, 1.0);
				Clear(COLOR_BUFFER_BIT);

				BindBuffer(ARRAY_BUFFER, vbo);
				BufferSubData(ARRAY_BUFFER, 0x0, size_of_val(& vtx) as GLsizeiptr, addr_of!(vtx) as *const c_void);

				UseProgram(gfx.shdprg);
				BindVertexArray(vao);
				DrawArrays(TRIANGLES, 0x0, 0x3*0x1);
			}

			gfx.win.swap_buffers();
		}

		return -0x45;
	}
}
