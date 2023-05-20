// Copyright 2023 Gabriel Jensen.

use crate::dw::app::{App,GOTINT};

extern crate libc;

use libc::{c_int,sighandler_t,signal,SIGINT,SIGTERM};
use std::mem::transmute;
use std::sync::atomic::Ordering;

fn hnd(sig: c_int) {
	unsafe {
		signal(sig,transmute::<fn(c_int),sighandler_t>(hnd));

		GOTINT.store(true,Ordering::Relaxed);
	}
}

impl App {
	pub fn inisig(&mut self) {
		eprintln!("initialising signal handlers");

		unsafe {
			signal(SIGINT, transmute::<fn(c_int),sighandler_t>(hnd));
			signal(SIGTERM,transmute::<fn(c_int),sighandler_t>(hnd));
		}
	}
}
