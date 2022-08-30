#![allow(unused_imports)]
#![allow(non_camel_case_types)]
// mod macros;
// use crate::macros::*;

// #[macro_use]
// extern crate redhook;
use redhook::*;

use std::borrow::Cow;
use std::ffi::CStr;
use std::env;
use lazy_static::lazy_static;

use libc::{uid_t, c_int};

// type opaque_ptr = *const ();
type string_ptr = *const libc::c_char;

// // Emulate C's `__attribute__((constructor))`
// // Under the hood it's just a section with `.init_array` and `.init_array_end` ASM sections.
// #[no_mangle]
// // #[link_section = ".init_array"] // Linux
// #[link_section = "__DATA,__mod_init_func"] // MacOS
// pub static LD_PRELOAD_INITIALISE_RUST: extern "C" fn() = self::ld_preload_init;
// extern "C" fn ld_preload_init() {
// 	dbg!("INIT");
// }


const REMAP_SRC: &'static [u8] = "/nix/".as_bytes();
lazy_static! {
	static ref REMAP: Remap = Remap::from_env();
}


struct Remap {
	src: &'static [u8],
	dest_prefix: Vec<u8>,
}

impl Remap {
	pub fn from_env() -> Self {
		let base = env::var("RUNIX_ROOT").unwrap_or_else(|_| panic!("$RUNIX_ROOT required")).into_bytes();
		Self::for_dest(base)
	}
	
	pub fn for_dest<V: Into<Vec<u8>>>(dest_prefix: V) -> Self {
		let dest_bytes = dest_prefix.into();
		if !dest_bytes.starts_with(&['/' as u8]) {
			panic!("RUNIX_ROOT doesn't begin with / [{}]", String::from_utf8(dest_bytes).unwrap());
		}

		Self {
			src: REMAP_SRC,
			dest_prefix: dest_bytes,
		}
	}
	
	pub fn intercept<'a>(&self, bytes: &'a [u8]) -> Cow<'a, [u8]> {
		if bytes.starts_with(self.src) {
			// dbg!("remapping");
			// dbg!(String::from_utf8(self.dest.clone()));
			let mut vec: Vec<u8> = Vec::with_capacity(bytes.len() + self.dest_prefix.len());
			vec.extend(&self.dest_prefix);
			vec.extend(bytes);
			Cow::Owned(vec)
		} else {
			Cow::Borrowed(bytes)
		}
	}
}

fn intercept<F, R>(s: string_ptr, func: F) -> R where F: FnOnce(string_ptr) -> R {
	unsafe {
		let orig = CStr::from_ptr(s).to_bytes();
		dbg!(String::from_utf8(orig.into()).unwrap());
		let cow = REMAP.intercept(orig);
		dbg!(String::from_utf8(cow.to_vec()).unwrap());
		let bytes = cow.as_ref();
		let cstr = CStr::from_bytes_with_nul_unchecked(bytes);
		func(cstr.as_ptr())
	}
}

hook! { unsafe fn open(path: string_ptr, flags: libc::c_int, mode: libc::mode_t) -> libc::c_int => wrapped_open {
	intercept(path, |p| open(p, flags, mode))
}}

// hook! { unsafe fn getuid() -> uid_t => wrapped_getuid {
// 	dbg!("getuid() -> 0");
// 	0
// }}
