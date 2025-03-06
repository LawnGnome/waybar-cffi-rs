use std::{ffi::CString, str::Utf8Error};

use thiserror::Error;
use waybar_cffi_sys::gtk::glib::BoolError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("gtk init error: {0}")]
    Gtk(#[source] BoolError),

    #[error("init info is null")]
    InitInfoNull,

    #[error(transparent)]
    Jsonc(#[from] serde_jsonc::Error),

    #[error("key {key:?} is invalid UTF-8: {e}")]
    KeyInvalid {
        #[source]
        e: Utf8Error,
        key: CString,
    },

    #[error("value in key {key} is invalid UTF-8: {e}")]
    ValueInvalid {
        #[source]
        e: Utf8Error,
        key: String,
    },
}
