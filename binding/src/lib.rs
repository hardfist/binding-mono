#![deny(clippy::all)]
use napi::bindgen_prelude::{*};
use rspack_core::{options::{BindingOptions}, callback};
use napi::{Result,Error, JsFunction};
use napi_derive::napi;
use napi::{threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode}};
use napi::threadsafe_function::ThreadSafeResultContext;
#[napi]
pub fn build(env:napi::Env,options:BindingOptions) -> napi::JsUndefined{
  let loader: JsFunction = options.module.binding_uses.and_then(|x| x.loader).ok_or("err").map_err(|err| Error::new(napi::Status::GenericFailure, "failed".to_owned())).unwrap();
  loader.call(None, &[env.create_int32(2).unwrap()]);
  env.get_undefined().unwrap()
}
