#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::approx_constant)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::useless_transmute)]

pub use gtk;
pub use libc;

mod raw;
pub use raw::*;

/// The version of Waybar the bindings were built against.
pub static WAYBAR_VERSION: &str = include_str!("../WAYBAR_VERSION");
