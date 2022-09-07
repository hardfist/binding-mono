
#[cfg(feature="node-api")]
use napi_derive::napi;
#[cfg(feature="node-api")]
use napi::bindgen_prelude::ToNapiValue;

use serde::{Deserialize,Serialize};
#[cfg(not(feature="node-api"))]
#[derive(Debug,Default)]
#[derive(Deserialize,Serialize)]
pub enum Mode {
    #[default]
    DEVELOPMENT,
    PRODUCTION
}

#[cfg(feature="node-api")]
#[derive(Debug,Default)]
#[napi]
pub enum Mode {
    #[default]
    DEVELOPMENT,
    PRODUCTION
}

#[cfg(not(feature="node-api"))]
#[derive(Debug,Default)]
#[derive(Deserialize,Serialize)]
pub enum Target {
    #[default]
    ES3,
    ES5
}
#[cfg(feature="node-api")]
#[derive(Debug,Default)]
#[napi]
pub enum Target {
    #[default]
    ES3,
    ES5
}




#[cfg(feature="node-api")]
#[derive(Debug,Default)]
#[napi(object)]
pub struct CompilerOptions {
   pub mode: Mode,
   pub targets: Vec<Target>,
   pub context: String
}

#[cfg(not(feature="node-api"))]
#[derive(Deserialize,Debug,Default,Serialize)]
pub struct CompilerOptions {
   #[serde(default)]
   pub mode: Mode,
   #[serde(default)]
   pub targets: Vec<Target>,
   #[serde(default)]
   pub context: String
}