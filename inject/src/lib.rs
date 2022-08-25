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

type opaque_ptr = *const ();
type string_ptr = *const libc::c_char;

// Emulate C's `__attribute__((constructor))`
// Under the hood it's just a section with `.init_array` and `.init_array_end` ASM sections.
#[no_mangle]
// #[link_section = ".init_array"] // Linux
#[link_section = "__DATA,__mod_init_func"] // MacOS
pub static LD_PRELOAD_INITIALISE_RUST: extern "C" fn() = self::ld_preload_init;
extern "C" fn ld_preload_init() {
	dbg!("INIT");
}


const REMAP_SRC: &'static [u8] = "/nix/".as_bytes();
lazy_static! {
	static ref REMAP: Remap = Remap::from_env();
}


struct Remap {
	src: &'static [u8],
	dest: Vec<u8>,
	diff: usize
}

impl Remap {
	pub fn from_env() -> Self {
		let mut base = env::var("RUNIX_ROOT").unwrap_or_else(|k|panic!("${} required", k)).into_bytes();
		if !base.ends_with(&['/' as u8]) {
			base.push('/' as u8);
		}
		Self::for_dest(base)
	}
	
	pub fn for_dest<V: Into<Vec<u8>>>(dest: V) -> Self {
		let dest_bytes = dest.into();
		let diff: usize = if dest_bytes.len() > REMAP_SRC.len() {
			dest_bytes.len() - REMAP_SRC.len()
		} else {
			panic!("RUNIX_ROOT must be at least {} bytes; got {}", REMAP_SRC.len(), dest_bytes.len());
		};

		Self {
			src: REMAP_SRC,
			dest: dest_bytes,
			diff
		}
	}
	
	pub fn intercept<'a>(&self, bytes: &'a [u8]) -> Cow<'a, [u8]> {
		if bytes.starts_with(self.src) {
			// dbg!("remapping");
			// dbg!(String::from_utf8(self.dest.clone()));
			let suffix = &bytes[self.src.len()..];
			let mut vec: Vec<u8> = Vec::with_capacity(bytes.len() + self.diff);
			vec.extend(&self.dest);
			vec.extend(suffix);
			Cow::Owned(vec)
		} else {
			Cow::Borrowed(bytes)
		}
	}
}

fn intercept<F, R>(s: string_ptr, func: F) -> R where F: FnOnce(string_ptr) -> R {
	unsafe {
		let orig = CStr::from_ptr(s).to_bytes();
		dbg!(String::from_utf8(orig.into()));
		let cow = REMAP.intercept(orig);
		dbg!(String::from_utf8(cow.to_vec()));
		let bytes = cow.as_ref();
		let cstr = CStr::from_bytes_with_nul_unchecked(bytes);
		func(cstr.as_ptr())
	}
}

hook! { unsafe fn open(path: string_ptr, flags: libc::c_int, mode: libc::mode_t) -> libc::c_int => wrapped_open {
	intercept(path, |p| open(p, flags, mode))
}}

hook! { unsafe fn getuid() -> uid_t => wrapped_getuid {
	dbg!("getuid() -> 0");
	0
}}
