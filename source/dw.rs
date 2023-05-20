// Copyright 2023 Gabriel Jensen.

pub struct VerTyp {
	pub maj: u64,
	pub min: u64,
	pub pat: u64,
}

pub const VER: VerTyp = VerTyp {
	maj: 0x0,
	min: 0x1,
	pat: 0x0,
};

pub const fn datpth() -> &'static str {
	if cfg!(unix) {
		//return "/usr/share/local/deltaworld";
		return "/home/delta/Repositories/deltaworld"
	}
	return "";
}

pub mod app;
