// Copyright 2023 Gabriel Jensen.

use crate::dw::app::App;

extern crate gl;

use gl::{AttachShader, COMPILE_STATUS, CompileShader, COMPUTE_SHADER, CreateProgram, CreateShader, DeleteShader, FALSE, FRAGMENT_SHADER, GEOMETRY_SHADER, GetShaderInfoLog, GetShaderiv, INFO_LOG_LENGTH, LinkProgram, ShaderSource, TESS_CONTROL_SHADER, TESS_EVALUATION_SHADER, VERTEX_SHADER};
use gl::types::{GLchar, GLint, GLsizei, GLuint};
use libc::{STDERR_FILENO, write};
use std::alloc::{alloc, dealloc, Layout};
use std::ffi::c_void;
use std::ptr::{addr_of, null};

impl App {
	pub fn getshdprg(&mut self) -> GLuint {
		eprintln!("compiling shaders");

		let cmpshd = |nam: & str| -> GLuint {
			let extoff = nam.find('.').expect("unable to find file extension seperator")+0x1;
			let filext = &nam[extoff..];
			let typ = match filext {
				"comp" => COMPUTE_SHADER,
				"frag" => FRAGMENT_SHADER,
				"geom" => GEOMETRY_SHADER,
				"tesc" => TESS_CONTROL_SHADER,
				"tese" => TESS_EVALUATION_SHADER,
				"vert" => VERTEX_SHADER,
				_      => panic!("invalid shader file extension \"{}\"", filext),
			};

			let typstr = match typ {
				FRAGMENT_SHADER => "fragment",
				VERTEX_SHADER   => "vertex",
				_               => unreachable!(),
			};

			eprintln!("compiling {} shader \"{}\"", typstr, nam);

			//let src = read(pth).expect("unable to read shader at");
			let src = match nam {
				"main.frag" => include_bytes!("shader/main.frag.glsl").to_vec(),
				"main.vert" => include_bytes!("shader/main.vert.glsl").to_vec(),
				_           => panic!("shader not found"),
			};

			unsafe {
				let shd    = CreateShader(typ);
				let srcptr = src.as_ptr();
				ShaderSource(shd, 0x1, addr_of!(srcptr) as *const *const GLchar, null::<GLint>());

				CompileShader(shd);

				let mut sts: GLint = 0x0;
				GetShaderiv(shd, COMPILE_STATUS, &mut sts);
				if sts == FALSE as GLint {
					let mut len: GLint = 0x0;
					GetShaderiv(shd, INFO_LOG_LENGTH, &mut len);
					
					let lay = Layout::from_size_align(len as usize, 0x1).unwrap();

					let log = alloc(lay);
					GetShaderInfoLog(shd, len, null::<GLsizei>() as *mut GLsizei, log as *mut GLchar);

					eprint!("unable able to compile shader:\n");
					write(STDERR_FILENO, log as *const c_void, len as usize);
					eprintln!();

					dealloc(log, lay);

					panic!("unable to compile shader");
				}

				return shd;
			}
		};

		let frgshd = cmpshd("main.frag");
		let vtxshd = cmpshd("main.vert");
		
		unsafe {
			let prg = CreateProgram();
			AttachShader(prg, frgshd);
			AttachShader(prg, vtxshd);
			LinkProgram(prg);

			DeleteShader(frgshd);
			DeleteShader(vtxshd);

			return prg;
		}
	}
}
