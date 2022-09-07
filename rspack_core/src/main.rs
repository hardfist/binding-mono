mod options;
use options::{CompilerOptions,Target,Mode};
pub fn build(options_str:&str ){
    let options: CompilerOptions = serde_json::from_str(options_str).unwrap();
    dbg!(options);
}
pub fn main(){
    let x = CompilerOptions{
        mode: Mode::DEVELOPMENT,
        targets: vec![Target::ES3,Target::ES5],
        context: "hello world".to_owned()
    };
    let str = serde_json::to_string(&x).unwrap();
    dbg!(&str);

    build("{}");
    build(r#"{"mode": "DEVELOPMENT"}"#);
    build(r#"{"mode": "DEVELOPMENT", "targets": ["ES3","ES5"]}"#);
}