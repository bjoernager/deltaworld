// Copyright 2023 Gabriel Jensen.

use crate::dw::{app::App,datpth};

extern crate gl;

use gl::{AttachShader,COMPILE_STATUS,CompileShader,CreateProgram,CreateShader,DeleteShader,FALSE,FRAGMENT_SHADER,GetShaderiv,LinkProgram,ShaderSource,VERTEX_SHADER};
use gl::types::{GLchar,GLenum,GLint,GLuint};
use std::fs::read;
use std::ptr::{addr_of,null};

impl App {
	pub fn getshdprg(&mut self) -> GLuint {
		eprintln!("compiling shaders");

		let cmpshd = |nam: & str,typ:GLenum| -> GLuint {
			let typstr = match typ {
				FRAGMENT_SHADER => "fragment",
				VERTEX_SHADER   => "vertex",
				_               => panic!("invalid shader type {}",typ),
			};

			let filext = match typ {
				FRAGMENT_SHADER => "frag",
				VERTEX_SHADER   => "vert",
				_               => panic!("invalid shader type {}",typ),
			};

			let mut pth = String::new();
			pth.push_str(datpth());
			pth.push_str("/shader/");
			pth.push_str(nam);
			pth.push(    '.');
			pth.push_str(filext);
			pth.push_str(".glsl");

			eprintln!("compiling {} shader at \"{}\"",typstr,pth);

			let src = read(pth).expect("unable to read shader at");

			unsafe {
				let shd    = CreateShader(typ);
				let srcptr = src.as_ptr();
				ShaderSource(shd,0x1,addr_of!(srcptr) as *const *const GLchar,null::<GLint>());

				CompileShader(shd);

				let mut sts:GLint = 0x0;
				GetShaderiv(shd,COMPILE_STATUS,&mut sts);
				if sts == FALSE as GLint {panic!("unable to compile shader");}

				return shd;
			}
		};

		let frgshd = cmpshd("main",FRAGMENT_SHADER);
		let vtxshd = cmpshd("main",VERTEX_SHADER);
		
		unsafe {
			let prg = CreateProgram();
			AttachShader(prg,frgshd);
			AttachShader(prg,vtxshd);
			LinkProgram(prg);

			DeleteShader(frgshd);
			DeleteShader(vtxshd);

			return prg;
		}
	}
}
