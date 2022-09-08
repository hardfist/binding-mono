#![deny(clippy::all)]
use rspack_core::options::{BindingOptions};
use napi::{Result};
use napi_derive::napi;

#[napi]
pub fn build(options:BindingOptions) -> Result<()>{
  dbg!(options);
  Ok(())
}
