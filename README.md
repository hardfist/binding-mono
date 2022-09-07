# binding demo
如何同时处理serde和napi的binging问题，减小config的处理流程
* binding: node-js binding只负责 CompilerOptions的binding
* cli: 负责js的normalize，传给binding的结果是已经normalize的bindingConfig
* core: rust侧读取配置文件，normalize的过程交给 serde 完成
* binding_options: 同时提供serde binding和napi binding