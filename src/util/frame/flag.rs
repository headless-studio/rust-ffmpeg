use libc::c_int;
use crate::ffi::*;

bitflags! {
	pub struct Flags: c_int {
		const CORRUPT = AV_FRAME_FLAG_CORRUPT;
	}
}
