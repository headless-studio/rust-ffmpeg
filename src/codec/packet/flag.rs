use ffi::*;
use libc::c_int;

bitflags! {
    pub struct Flags: c_int {
        pub const KEY     = AV_PKT_FLAG_KEY;
        pub const CORRUPT = AV_PKT_FLAG_CORRUPT;
    }
}
