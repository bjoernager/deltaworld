// Copyright 2023 Gabriel Jensen.

pub struct VerTyp {
	pub maj: u64,
	pub min: u64,
	pub pat: u64,
}

pub const VER: VerTyp = VerTyp {
	maj: 0x0,
	min: 0x0,
	pat: 0x0,
};

pub mod app;
