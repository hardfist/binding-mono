
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
type Generator = Box<dyn Fn() -> ()>;


#[derive(Deserialize,Serialize,Default)]
pub struct Module {
    #[serde(skip_deserializing,skip_serializing)]
    generator: Option<Generator>,
    test: String
}
impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module").field("generator", &"generator").field("test", &self.test).finish()
    }
}

#[cfg_attr(feature="node-api",napi(object))]
#[derive(Debug,Default,Deserialize,Serialize)]
pub struct BindingOptions {
   #[serde(default)]
   pub mode: Mode,
   #[serde(default)]
   pub targets: Vec<Target>,
   #[serde(default)]
   pub context: String,
  #[serde(default)]
   pub module: Module
}
