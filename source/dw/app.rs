// Copyright 2023 Gabriel Jensen.

extern crate glfw;

use glfw::{Glfw,Window,WindowEvent};
use std::sync::mpsc::Receiver;

pub struct Gfx {
	evt:  Receiver<(f64,WindowEvent)>,
	glfw: Glfw,
	win:  Window,
}

pub struct App {
}

pub mod ini;
pub mod inigfx;
pub mod lop;
