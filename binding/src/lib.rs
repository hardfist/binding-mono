#![deny(clippy::all)]
use rspack_core::options::{CompilerOptions};
use napi::{Result};
use napi_derive::napi;

#[napi]
pub fn build(options:CompilerOptions) -> Result<()>{
  dbg!(options);
  Ok(())
}
