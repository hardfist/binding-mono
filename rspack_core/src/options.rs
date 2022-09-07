
#[cfg(feature="node-api")]
use napi_derive::napi;
#[cfg(feature="node-api")]
use napi::bindgen_prelude::ToNapiValue;

use serde::{Deserialize,Serialize};

#[cfg_attr(feature="node-api",napi)]
#[derive(Debug,Default,Deserialize,Serialize)]
pub enum Mode {
    #[default]
    DEVELOPMENT,
    PRODUCTION
}

#[cfg_attr(feature="node-api",napi)]
#[derive(Debug,Default,Deserialize,Serialize)]
pub enum Target {
    #[default]
    ES3,
    ES5
}
#[cfg_attr(feature="node-api",napi(object))]
#[derive(Debug,Default,Deserialize,Serialize)]
pub struct CompilerOptions {
   #[serde(default)]
   pub mode: Mode,
   #[serde(default)]
   pub targets: Vec<Target>,
   #[serde(default)]
   pub context: String
}
