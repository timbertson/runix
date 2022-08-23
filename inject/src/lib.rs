#[macro_use]
extern crate redhook;

use libc::uid_t;

hook! {
	unsafe fn getuid() -> uid_t => i_am_root {
		dbg!("getuid() -> 0");
		0
	}
}
