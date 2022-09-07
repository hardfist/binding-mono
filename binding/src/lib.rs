#![deny(clippy::all)]
use binding_options::{CompilerOptions};
use napi::{Result};
use napi_derive::napi;

#[napi]
pub fn build(options:CompilerOptions) -> Result<()>{
  dbg!(options);
  Ok(())
}
