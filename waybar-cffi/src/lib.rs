//! A crate that allows Waybar modules to be built in Rust.
//!
//! Waybar modules are written using its experimental CFFI module interface, and
//! implement their UI using [Gtk 3][`gtk`]. Modules are built as a shared
//! library that is loaded by Waybar at runtime.
//!
//! This version of the bindings was built against Waybar version
#![doc = concat!(include_str!("../WAYBAR_VERSION"), ".")]
//!
//! ## Example
//!
//! Implementing a module is done by implementing the [`Module`] trait, and then
//! calling [`waybar_module`] to export the required symbols from the shared
//! library:
//!
//! ```
#![doc = include_str!("../examples/hello-world.rs")]
//! ```
//!
//! This example is also available in `examples/hello-world.rs`, and can be built and run with:
//!
//! ```bash
//! $ cargo build --example hello-world
//! $ waybar -c examples/hello-world.json
//! ```
//!
//! ## Gtk
//!
//! Waybar still uses Gtk 3 for its UI, so modules are required to also use it.
//!
//! This crate re-exports the [`gtk`] crate, and it is strongly suggested that
//! you use this re-export, rather than including `gtk` as a dependency in your
//! own crate.
//!
//! ### Async
//!
//! If you need async functionality, you should use
//! [`glib::MainContext`][gtk::glib::MainContext] as the reactor via methods
//! like [`MainContext::spawn_local`][gtk::glib::MainContext::spawn_local],
//! rather than trying to start up a new async runtime. This will ensure that
//! your module integrates properly with the main Waybar event loop.

#[doc(hidden)]
pub mod config;
mod error;
mod info;

#[doc(hidden)]
pub use crate::error::Error;
pub use crate::info::InitInfo;
#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use waybar_cffi_sys as sys;
pub use waybar_cffi_sys::gtk;

use serde::de::DeserializeOwned;
use waybar_cffi_sys::{
    libc::{c_void, size_t},
    wbcffi_config_entry, wbcffi_init_info,
};

/// A Waybar CFFI module.
///
/// In most cases, only [`Module::init`] needs to be specified, and the default
/// implementations of the other methods can be used unchanged.
#[allow(unused_variables)]
pub trait Module {
    /// The configuration type.
    ///
    /// The JSONC configuration for the module will be deserialised using
    /// [`serde`] into a value of this type.
    type Config: DeserializeOwned;

    /// Called when the module is being initialised.
    ///
    /// Generally, you'll want to create your UI using [`gtk`] here, attaching
    /// it to the container returned by [`InitInfo::get_root_widget`].
    fn init(info: &InitInfo, config: Self::Config) -> Self;

    /// Called when the module should be updated.
    fn update(&mut self) {}

    /// Called when the module should be refreshed in response to a signal.
    fn refresh(&mut self, signal: i32) {}

    /// Called when an action is called on the module.
    fn do_action(&mut self, action: &str) {}
}

/// Defines and exports the C functions required for the module to be used by
/// Waybar.
///
/// This macro must be invoked exactly once in a Waybar module crate.
#[macro_export]
macro_rules! waybar_module {
    ($ty:ty) => {
        #[allow(non_upper_case_globals)]
        #[unsafe(no_mangle)]
        pub static wbcffi_version: $crate::sys::libc::size_t = 1;

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        #[unsafe(no_mangle)]
        pub extern "C" fn wbcffi_init(
            init_info: *const $crate::sys::wbcffi_init_info,
            config_entries: *const $crate::sys::wbcffi_config_entry,
            config_entries_len: $crate::sys::libc::size_t,
        ) -> *mut $crate::sys::libc::c_void {
            $crate::init::<$ty>(
                stringify!($ty),
                init_info,
                config_entries,
                config_entries_len,
            )
        }

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        #[unsafe(no_mangle)]
        pub extern "C" fn wbcffi_deinit(instance: *mut $crate::sys::libc::c_void) {
            let ptr = instance as *mut $ty;
            let module = unsafe { Box::from_raw(ptr) };

            // Intentionally not converted back to raw, as we want the module to be dropped.
        }

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        #[unsafe(no_mangle)]
        pub extern "C" fn wbcffi_update(instance: *mut $crate::sys::libc::c_void) {
            let ptr = instance as *mut $ty;
            let mut module = unsafe { Box::from_raw(ptr) };

            module.update();

            let _ = Box::into_raw(module);
        }

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        #[unsafe(no_mangle)]
        pub extern "C" fn wbcffi_refresh(
            instance: *mut $crate::sys::libc::c_void,
            signal: $crate::sys::libc::c_int,
        ) {
            let ptr = instance as *mut $ty;
            let mut module = unsafe { Box::from_raw(ptr) };

            module.refresh(signal as i32);

            let _ = Box::into_raw(module);
        }

        #[allow(clippy::not_unsafe_ptr_arg_deref)]
        #[unsafe(no_mangle)]
        pub extern "C" fn wbcffi_doaction(
            instance: *mut $crate::sys::libc::c_void,
            action_name: *const $crate::sys::libc::c_char,
        ) {
            let ptr = instance as *mut $ty;
            let mut module = unsafe { Box::from_raw(ptr) };

            let action = unsafe { std::ffi::CStr::from_ptr(action_name) }
                .to_str()
                .expect("action name");
            module.do_action(action);

            let _ = Box::into_raw(module);
        }
    };
}

#[doc(hidden)]
pub fn init<T: Module>(
    name: &'static str,
    init_info: *const wbcffi_init_info,
    config_entries: *const wbcffi_config_entry,
    config_entries_len: size_t,
) -> *mut c_void {
    // A tiny wrapper to avoid having to do too much from within the macro
    // definition.
    match do_init::<T>(init_info, config_entries, config_entries_len) {
        Ok(module) => Box::into_raw(Box::new(module)) as *mut c_void,
        Err(e) => {
            eprintln!("{name} init error: {e}");
            std::ptr::null_mut()
        }
    }
}

fn do_init<T: Module>(
    init_info: *const wbcffi_init_info,
    config_entries: *const wbcffi_config_entry,
    config_entries_len: size_t,
) -> Result<T, Error> {
    // This has to happen somewhere. Here's as good as anywhere, and at least we
    // know it's early.
    gtk::init().map_err(Error::Gtk)?;

    let info = InitInfo::new(init_info)?;
    let config = config::parse(config_entries, config_entries_len)?;

    Ok(T::init(&info, config))
}
