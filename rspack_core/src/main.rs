mod options;
use options::{BindingOptions,Target,Mode,Module};
pub fn build(options_str:&str ){
    let options: BindingOptions = serde_json::from_str(options_str).unwrap();
    dbg!(options);
}
pub fn main(){
    let mut options = BindingOptions{
        mode: Mode::DEVELOPMENT,
        targets: vec![Target::ES3,Target::ES5],
        context: "hello world".to_owned(),
        module: Module::default()
    };
    
    let str = serde_json::to_string(&options).unwrap();
    dbg!(&str);

    build("{}");
    build(r#"{"mode": "DEVELOPMENT"}"#);
    build(r#"{"mode": "DEVELOPMENT", "targets": ["ES3","ES5"]}"#);
}