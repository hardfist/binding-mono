"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.build = void 0;
const binding = require('binding');
var Mode;
(function (Mode) {
    Mode[Mode["DEVELOPMENT"] = 0] = "DEVELOPMENT";
    Mode[Mode["PRODUCTION"] = 1] = "PRODUCTION";
})(Mode || (Mode = {}));
var Target;
(function (Target) {
    Target[Target["ES3"] = 0] = "ES3";
    Target[Target["ES5"] = 1] = "ES5";
})(Target || (Target = {}));
function resolveMode(mode) {
    if (!mode) {
        return Mode.DEVELOPMENT;
    }
    if (mode === 'development') {
        return Mode.DEVELOPMENT;
    }
    else if (mode === 'production') {
        return Mode.PRODUCTION;
    }
    else {
        throw new Error('not supported');
    }
}
function resolveTarget(target) {
    if (!target) {
        return [];
    }
    return target.map(x => {
        if (x == 'es3' || x == "ES3") {
            return Target.ES3;
        }
        else if (x === 'es5' || x === 'ES5') {
            return Target.ES5;
        }
        else {
            throw new Error("not supported");
        }
    });
}
function normalizeConfig(config) {
    var _a;
    return {
        mode: resolveMode(config.mode),
        targets: resolveTarget(config.target),
        context: (_a = config.context) !== null && _a !== void 0 ? _a : "",
        module: {
            generator: "",
            test: ""
        },
    };
}
function build(raw_config) {
    const bindingConfig = normalizeConfig(raw_config);
    console.log('bindingConfig:', bindingConfig);
    binding.build(bindingConfig);
}
exports.build = build;
