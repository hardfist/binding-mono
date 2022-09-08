use std::{ptr, default, fmt::Debug};
#[cfg(feature="node-api")]
use napi::{sys, bindgen_prelude::{ToNapiValue, FromNapiValue},Env,Result,  JsFunction};
#[cfg(feature="node-api")]
use napi_derive::napi;
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
#[derive(Default,Deserialize,Serialize)]
#[cfg_attr(feature="node-api",napi(object))]
pub struct ModuleUseRule {
    #[serde(skip_deserializing,skip_serializing)]
    
    pub loader: Option<JsFunction>
}
impl Debug for ModuleUseRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleUseRule").field("loader", &"loader").finish()
    }
}
#[cfg_attr(feature="node-api",napi(object))]
#[derive(Deserialize,Serialize,Default)]
pub struct Module {
    #[serde(skip_deserializing,skip_serializing)]
    pub uses: Option<Callback>, // this is rust side uses, which will convert binding_uses to uses
    #[serde(skip_deserializing,skip_serializing)]
    pub binding_uses: Option<ModuleUseRule>,
    pub test: String
}
impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module").field("binding_uses", &self.binding_uses).field("test", &self.test).finish()
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