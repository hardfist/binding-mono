const adapter = require('./adapter');
console.log({adapter});
adapter.build({

})
adapter.build({
  mode: "development"
})
adapter.build({
  target: ["es3","es5"]
})
adapter.build({
  mode: "development"
})
adapter.build({
  mode: "production",
  target: ["es3","es5"]
})