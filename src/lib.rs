#![allow(non_camel_case_types)]

extern crate libc;
pub extern crate ffmpeg_sys as sys;
#[macro_use] extern crate bitflags;
#[cfg(feature = "image")] extern crate image;

pub use crate::sys as ffi;

#[macro_use]
pub mod util;
pub use crate::util::error::Error;
pub use crate::util::dictionary;
pub use crate::util::dictionary::Owned as Dictionary;
pub use crate::util::dictionary::Ref as DictionaryRef;
pub use crate::util::dictionary::Mut as DictionaryMut;
pub use crate::util::rational::{self, Rational};
pub use crate::util::media;
pub use crate::util::picture;
pub use crate::util::color;
pub use crate::util::chroma;
pub use crate::util::time;
pub use crate::util::frame::{self, Frame};
pub use crate::util::channel_layout::{self, ChannelLayout};
pub use crate::util::option;
pub use crate::util::mathematics::{self, Rounding, Rescale, rescale};

#[cfg(feature = "format")]
pub mod format;
#[cfg(feature = "format")]
pub use crate::format::format::Format;
#[cfg(feature = "format")]
pub use crate::format::stream::{Stream, StreamMut};
#[cfg(feature = "format")]
pub use crate::format::chapter::{Chapter, ChapterMut};

#[cfg(feature = "codec")]
pub mod codec;
#[cfg(feature = "codec")]
pub use crate::codec::packet::{self, Packet};
#[cfg(feature = "codec")]
pub use crate::codec::subtitle::{self, Subtitle};
#[cfg(feature = "codec")]
pub use crate::codec::picture::Picture;
#[cfg(feature = "codec")]
pub use crate::codec::discard::Discard;
#[cfg(feature = "codec")]
pub use crate::codec::codec::Codec;
#[cfg(feature = "codec")]
pub use crate::codec::{decoder, encoder};
#[cfg(feature = "codec")]
pub use crate::codec::field_order::FieldOrder;
#[cfg(feature = "codec")]
pub use crate::codec::audio_service::AudioService;
#[cfg(feature = "codec")]
pub use crate::codec::threading;

#[cfg(feature = "device")]
pub mod device;

#[cfg(feature = "filter")]
pub mod filter;
#[cfg(feature = "filter")]
pub use crate::filter::Filter;

pub mod software;

fn init_error() {
	util::error::register_all();
}

#[cfg(feature = "format")]
fn init_format() {
	format::register_all();
}

#[cfg(not(feature = "format"))]
fn init_format() { }

#[cfg(feature = "device")]
fn init_device() {
	device::register_all();
}

#[cfg(not(feature = "device"))]
fn init_device() { }

#[cfg(feature = "filter")]
fn init_filter() {
	filter::register_all();
}

#[cfg(not(feature = "filter"))]
fn init_filter() { }

pub fn init() -> Result<(), Error> {
	init_error();
	init_format();
	init_device();
	init_filter();

	Ok(())
}
