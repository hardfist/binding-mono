const binding = require('binding');
enum Mode {
  DEVELOPMENT,
  PRODUCTION
}
enum Target {
  ES3,
  ES5
}
export interface RawConfig {
  mode?: string,
  target?: string[],
  context?: string
}
export interface BindingConfig {
  mode: Mode,
  targets: Target[],
  context: string,
  module: any,
}
function resolveMode(mode?: string){
  if(!mode){
    return Mode.DEVELOPMENT
  }
  if(mode === 'development'){
    return Mode.DEVELOPMENT
  }else if(mode === 'production'){
    return Mode.PRODUCTION
  }else {
    throw new Error('not supported')
  }
}
function resolveTarget(target?:string[]){
  if(!target){
    return [];
  }
  return target.map(x => {
    if(x == 'es3' || x == "ES3"){
      return Target.ES3
    }else if (x === 'es5' || x === 'ES5'){
      return Target.ES5
    }else {
      throw new Error("not supported")
    }
  })
}
function normalizeConfig(config: RawConfig): BindingConfig{
  return {
    mode: resolveMode(config.mode),
    targets: resolveTarget(config.target),
    context: config.context ?? "",
    module: {
      generator: "",
      test: ""
    },
  }
}

export function build(raw_config: RawConfig){
  const bindingConfig = normalizeConfig(raw_config);
  console.log('bindingConfig:', bindingConfig);
  binding.build(bindingConfig);
}