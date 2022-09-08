use std::{ptr, default};
#[cfg(feature="node-api")]
use napi::{Env,Result};
#[cfg(feature="node-api")]
use napi_derive::napi;
#[cfg(feature="node-api")]
use napi::bindgen_prelude::{ToNapiValue,FromNapiValue};
#[cfg(feature="node-api")]
use napi::sys;
pub struct Callback(Option<Box<dyn Fn() -> ()>>);
impl Default for Callback{
    fn default() -> Self {
       Self{0: None}
    }
}
#[cfg(feature="node-api")]
impl ToNapiValue for Callback{
unsafe fn to_napi_value(env: sys::napi_env, val: Self) -> Result<sys::napi_value> {
    let mut ptr = ptr::null_mut();

    Ok(ptr)
  }
}
#[cfg(feature="node-api")]
impl FromNapiValue for Callback{
  unsafe fn from_napi_value(env: sys::napi_env, napi_val: sys::napi_value) -> Result<Self> {
    let mut value = ptr::null_mut();
    let value = unsafe { Box::from_raw(value as *mut u32) };
    Ok(Callback(None))
  }
}