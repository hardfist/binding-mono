#![deny(clippy::all)]
use napi::bindgen_prelude::{Promise, Buffer};
use rspack_core::{options::{BindingOptions}, callback};
use napi::{Result,Error, JsFunction};
use napi_derive::napi;
use napi::{threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode}};
use napi::threadsafe_function::ThreadSafeResultContext;
#[napi]
pub fn build(options:BindingOptions) -> Result<()>{
  let loader: JsFunction = options.module.binding_uses.and_then(|x| x.loader).ok_or("err").map_err(|err| Error::new(napi::Status::GenericFailure, "failed".to_owned()))?;
  let tsfn: ThreadsafeFunction<u32, ErrorStrategy::CalleeHandled> = loader
    .create_threadsafe_function(0, |ctx| {
      ctx.env.create_uint32(ctx.value + 1).map(|v| vec![v])
    },|ctx: ThreadSafeResultContext<Promise<Buffer>>| {
                      let return_value = ctx.return_value;

                      ctx
                        .env
                        .execute_tokio_future(
                          async move {
                            Ok(())
                          },
                          |_env, ret| Ok(ret),
                        )
                        .expect("failed to execute tokio future");
                    },)?;

   tsfn.call(Ok(100), ThreadsafeFunctionCallMode::Blocking);

  Ok(())
}
