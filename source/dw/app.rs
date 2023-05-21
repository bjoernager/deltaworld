// Copyright 2023 Gabriel Jensen.

extern crate glfw;

use gl::types::GLuint;
use glfw::{Glfw, Window, WindowEvent};
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Receiver;

pub struct Gfx {
	evt:    Receiver<(f64, WindowEvent)>,
	glfw:   Glfw,
	shdprg: GLuint,
	win:    Window,
}

pub struct App {
}

static mut GOTINT: AtomicBool = AtomicBool::new(false);

pub mod getshdprg;
pub mod end;
pub mod ini;
pub mod inigfx;
pub mod inisig;
pub mod lop;
pub mod new;
