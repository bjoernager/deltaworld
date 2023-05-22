// Copyright 2023 Gabriel Jensen.

pub struct Vertyp<T> {
	pub maj: T,
	pub min: T,
	pub pat: T,
}

pub const VER: Vertyp::<u64> = Vertyp::<u64> {
	maj: 0x0,
	min: 0x3,
	pat: 0x0,
};

#[allow(dead_code)]
pub struct Pos<T> {
	x: T,
	y: T,
	z: T,
}

#[allow(dead_code)]
pub struct Cub {
	//pos
}

pub const fn datpth() -> &'static str {
	if cfg!(unix) {
		//return "/usr/share/local/deltaworld";
		return "/home/delta/Repositories/deltaworld"
	}
	else {
		panic!("data directory not found");
	}
}

pub mod app;
