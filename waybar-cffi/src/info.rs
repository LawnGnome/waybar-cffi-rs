use std::ffi::CStr;

use waybar_cffi_sys::{
    gtk::{
        ffi::GtkContainer,
        glib::translate::{from_glib_borrow, Borrowed},
        Container,
    },
    wbcffi_init_info,
};

use crate::Error;

/// Metadata provided by Waybar when initialising a module.
pub struct InitInfo(*const wbcffi_init_info);

impl InitInfo {
    pub(crate) fn new(ptr: *const wbcffi_init_info) -> Result<Self, Error> {
        if ptr.is_null() {
            Err(Error::InitInfoNull)
        } else {
            Ok(Self(ptr))
        }
    }

    /// Returns the root Gtk widget that the module should draw its own UI
    /// inside.
    pub fn get_root_widget(&self) -> Borrowed<Container> {
        let get_root_widget =
            unsafe { (*self.0).get_root_widget }.expect("get_root_widget is not null");
        let root = unsafe { get_root_widget((*self.0).obj) } as *mut GtkContainer;

        unsafe { from_glib_borrow(root) }
    }

    /// Returns the Waybar version.
    pub fn waybar_version(&self) -> &str {
        let version = unsafe { CStr::from_ptr((*self.0).waybar_version) };
        version.to_str().expect("valid UTF-8 in waybar version")
    }
}
