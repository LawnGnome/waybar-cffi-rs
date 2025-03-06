use std::{collections::HashMap, ffi::CStr};

use crate::Error;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use waybar_cffi_sys::{libc::size_t, wbcffi_config_entry};

pub fn parse<T>(ptr: *const wbcffi_config_entry, len: size_t) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    // This is kinda silly, but it does work: we take the pairs, serialise them
    // into JSONC, and then deserialise them back out.
    let pairs: HashMap<_, _> = (0..len).map(|i| entry(ptr, i)).try_collect()?;
    let raw = serde_jsonc::to_value(&pairs)?;

    Ok(serde_jsonc::from_value(raw)?)
}

fn entry(
    ptr: *const wbcffi_config_entry,
    offset: size_t,
) -> Result<(String, serde_jsonc::Value), Error> {
    unsafe {
        let entry = ptr.add(offset);

        let key = CStr::from_ptr((*entry).key);
        let key = match key.to_str() {
            Ok(s) => String::from(s),
            Err(e) => {
                return Err(Error::KeyInvalid {
                    e,
                    key: key.to_owned(),
                });
            }
        };
        let value =
            serde_jsonc::from_slice(CStr::from_ptr((*entry).value).to_bytes()).map_err(|e| {
                Error::ValueInvalid {
                    e,
                    key: key.clone(),
                }
            })?;

        Ok((key, value))
    }
}
