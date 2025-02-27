# [json_static.w](../../../../examples/tests/valid/json_static.w) | compile | tf-aws

## clients/$Inflight1.inflight.js
```js
module.exports = function({ jj }) {
  class  $Inflight1 {
    constructor({  }) {
    }
    async handle()  {
      {
        const ss = ((args) => { return JSON.stringify(args[0], null, args[1]) })([jj]);
        {((cond) => {if (!cond) throw new Error(`assertion failed: '(ss === "{\"a\":123,\"b\":{\"c\":456,\"d\":789}}")'`)})((ss === "{\"a\":123,\"b\":{\"c\":456,\"d\":789}}"))};
      }
    }
  }
  return $Inflight1;
}

```

## main.tf.json
```json
{
  "//": {
    "metadata": {
      "backend": "local",
      "stackName": "root",
      "version": "0.15.2"
    },
    "outputs": {
      "root": {
        "Default": {
          "cloud.TestRunner": {
            "TestFunctionArns": "WING_TEST_RUNNER_FUNCTION_ARNS"
          }
        }
      }
    }
  },
  "output": {
    "WING_TEST_RUNNER_FUNCTION_ARNS": {
      "value": "[[\"root/Default/Default/test:Access Json static inflight\",\"${aws_lambda_function.root_testAccessJsonstaticinflight_Handler_8BBF2CE3.arn}\"]]"
    }
  },
  "provider": {
    "aws": [
      {}
    ]
  },
  "resource": {
    "aws_iam_role": {
      "root_testAccessJsonstaticinflight_Handler_IamRole_9D202F56": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:Access Json static inflight/Handler/IamRole",
            "uniqueId": "root_testAccessJsonstaticinflight_Handler_IamRole_9D202F56"
          }
        },
        "assume_role_policy": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"sts:AssumeRole\",\"Principal\":{\"Service\":\"lambda.amazonaws.com\"},\"Effect\":\"Allow\"}]}"
      }
    },
    "aws_iam_role_policy": {
      "root_testAccessJsonstaticinflight_Handler_IamRolePolicy_06C4DEEC": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:Access Json static inflight/Handler/IamRolePolicy",
            "uniqueId": "root_testAccessJsonstaticinflight_Handler_IamRolePolicy_06C4DEEC"
          }
        },
        "policy": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Effect\":\"Allow\",\"Action\":\"none:null\",\"Resource\":\"*\"}]}",
        "role": "${aws_iam_role.root_testAccessJsonstaticinflight_Handler_IamRole_9D202F56.name}"
      }
    },
    "aws_iam_role_policy_attachment": {
      "root_testAccessJsonstaticinflight_Handler_IamRolePolicyAttachment_CF756084": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:Access Json static inflight/Handler/IamRolePolicyAttachment",
            "uniqueId": "root_testAccessJsonstaticinflight_Handler_IamRolePolicyAttachment_CF756084"
          }
        },
        "policy_arn": "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
        "role": "${aws_iam_role.root_testAccessJsonstaticinflight_Handler_IamRole_9D202F56.name}"
      }
    },
    "aws_lambda_function": {
      "root_testAccessJsonstaticinflight_Handler_8BBF2CE3": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:Access Json static inflight/Handler/Default",
            "uniqueId": "root_testAccessJsonstaticinflight_Handler_8BBF2CE3"
          }
        },
        "environment": {
          "variables": {
            "WING_FUNCTION_NAME": "Handler-c8867497"
          }
        },
        "function_name": "Handler-c8867497",
        "handler": "index.handler",
        "publish": true,
        "role": "${aws_iam_role.root_testAccessJsonstaticinflight_Handler_IamRole_9D202F56.arn}",
        "runtime": "nodejs18.x",
        "s3_bucket": "${aws_s3_bucket.root_Code_02F3C603.bucket}",
        "s3_key": "${aws_s3_object.root_testAccessJsonstaticinflight_Handler_S3Object_BE5E17BA.key}",
        "timeout": 30,
        "vpc_config": {
          "security_group_ids": [],
          "subnet_ids": []
        }
      }
    },
    "aws_s3_bucket": {
      "root_Code_02F3C603": {
        "//": {
          "metadata": {
            "path": "root/Default/Code",
            "uniqueId": "root_Code_02F3C603"
          }
        },
        "bucket_prefix": "code-c84a50b1-"
      }
    },
    "aws_s3_object": {
      "root_testAccessJsonstaticinflight_Handler_S3Object_BE5E17BA": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:Access Json static inflight/Handler/S3Object",
            "uniqueId": "root_testAccessJsonstaticinflight_Handler_S3Object_BE5E17BA"
          }
        },
        "bucket": "${aws_s3_bucket.root_Code_02F3C603.bucket}",
        "key": "<ASSET_KEY>",
        "source": "<ASSET_SOURCE>"
      }
    }
  }
}
```

## preflight.js
```js
const $stdlib = require('@winglang/sdk');
const $outdir = process.env.WING_SYNTH_DIR ?? ".";
const $wing_is_test = process.env.WING_IS_TEST === "true";
const $AppBase = $stdlib.core.App.for(process.env.WING_TARGET);
const cloud = require('@winglang/sdk').cloud;
class $Root extends $stdlib.std.Resource {
  constructor(scope, id) {
    super(scope, id);
    class $Inflight1 extends $stdlib.std.Resource {
      constructor(scope, id, ) {
        super(scope, id);
        this._addInflightOps("handle");
      }
      static _toInflightType(context) {
        const self_client_path = "./clients/$Inflight1.inflight.js".replace(/\\/g, "/");
        const jj_client = context._lift(jj);
        return $stdlib.core.NodeJsCode.fromInline(`
          require("${self_client_path}")({
            jj: ${jj_client},
          })
        `);
      }
      _toInflight() {
        return $stdlib.core.NodeJsCode.fromInline(`
          (await (async () => {
            const $Inflight1Client = ${$Inflight1._toInflightType(this).text};
            const client = new $Inflight1Client({
            });
            if (client.$inflight_init) { await client.$inflight_init(); }
            return client;
          })())
        `);
      }
      _registerBind(host, ops) {
        if (ops.includes("$inflight_init")) {
        }
        if (ops.includes("handle")) {
          this._registerBindObject(jj, host, []);
        }
        super._registerBind(host, ops);
      }
    }
    const x = Object.freeze({"a":123,"b":{"c":456,"d":789}});
    const k = (Object.keys(x));
    {((cond) => {if (!cond) throw new Error(`assertion failed: '(k.length === 2)'`)})((k.length === 2))};
    const v = (Object.values(x));
    {((cond) => {if (!cond) throw new Error(`assertion failed: '((v.at(0)) === 123)'`)})(((v.at(0)) === 123))};
    const m = (JSON.parse(JSON.stringify(x)));
    ((obj, args) => { obj[args[0]] = args[1]; })(m, ["a",321]);
    {((cond) => {if (!cond) throw new Error(`assertion failed: '((m)["a"] === 321)'`)})(((m)["a"] === 321))};
    let k2 = (Object.keys(m));
    {((cond) => {if (!cond) throw new Error(`assertion failed: '(k2.length === 2)'`)})((k2.length === 2))};
    ((args) => { delete (args[0])[args[1]]; })([m,"b"]);
    k2 = (Object.keys(m));
    {((cond) => {if (!cond) throw new Error(`assertion failed: '(k2.length === 1)'`)})((k2.length === 1))};
    const s = "{\"a\": 123, \"b\": {\"c\": 456, \"d\": 789}}";
    const j = (JSON.parse(s));
    {((cond) => {if (!cond) throw new Error(`assertion failed: '((Object.keys(j)).length === 2)'`)})(((Object.keys(j)).length === 2))};
    const invalidJson = "invalid";
    const tryParsed = (((args) => { try { return JSON.parse(args); } catch (err) { return undefined; } })(invalidJson) ?? Object.freeze({"key":"value"}));
    {((cond) => {if (!cond) throw new Error(`assertion failed: '((tryParsed)["key"] === "value")'`)})(((tryParsed)["key"] === "value"))};
    const jj = Object.freeze({"a":123,"b":{"c":456,"d":789}});
    const ss = ((args) => { return JSON.stringify(args[0], null, args[1]) })([jj]);
    {((cond) => {if (!cond) throw new Error(`assertion failed: '(ss === "{\"a\":123,\"b\":{\"c\":456,\"d\":789}}")'`)})((ss === "{\"a\":123,\"b\":{\"c\":456,\"d\":789}}"))};
    const ss2 = ((args) => { return JSON.stringify(args[0], null, args[1]) })([jj,2]);
    {((cond) => {if (!cond) throw new Error(`assertion failed: '(ss2 === "{\n  \"a\": 123,\n  \"b\": {\n    \"c\": 456,\n    \"d\": 789\n  }\n}")'`)})((ss2 === "{\n  \"a\": 123,\n  \"b\": {\n    \"c\": 456,\n    \"d\": 789\n  }\n}"))};
    const jsonOfMany = Object.freeze({"a":123,"b":"hello","c":true});
    {((cond) => {if (!cond) throw new Error(`assertion failed: '(((args) => { if (typeof args !== "string") {throw new Error("unable to parse " + typeof args + " " + args + " as a string")}; return JSON.parse(JSON.stringify(args)) })((jsonOfMany)["b"]) === "hello")'`)})((((args) => { if (typeof args !== "string") {throw new Error("unable to parse " + typeof args + " " + args + " as a string")}; return JSON.parse(JSON.stringify(args)) })((jsonOfMany)["b"]) === "hello"))};
    {((cond) => {if (!cond) throw new Error(`assertion failed: '(((args) => { if (typeof args !== "number") {throw new Error("unable to parse " + typeof args + " " + args + " as a number")}; return JSON.parse(JSON.stringify(args)) })((jsonOfMany)["a"]) === 123)'`)})((((args) => { if (typeof args !== "number") {throw new Error("unable to parse " + typeof args + " " + args + " as a number")}; return JSON.parse(JSON.stringify(args)) })((jsonOfMany)["a"]) === 123))};
    {((cond) => {if (!cond) throw new Error(`assertion failed: '((args) => { if (typeof args !== "boolean") {throw new Error("unable to parse " + typeof args + " " + args + " as a boolean")}; return JSON.parse(JSON.stringify(args)) })((jsonOfMany)["c"])'`)})(((args) => { if (typeof args !== "boolean") {throw new Error("unable to parse " + typeof args + " " + args + " as a boolean")}; return JSON.parse(JSON.stringify(args)) })((jsonOfMany)["c"]))};
    this.node.root.new("@winglang/sdk.cloud.Test",cloud.Test,this,"test:Access Json static inflight",new $Inflight1(this,"$Inflight1"));
  }
}
class $App extends $AppBase {
  constructor() {
    super({ outdir: $outdir, name: "json_static", plugins: $plugins, isTestEnvironment: $wing_is_test });
    if ($wing_is_test) {
      new $Root(this, "env0");
      const $test_runner = this.testRunner;
      const $tests = $test_runner.findTests();
      for (let $i = 1; $i < $tests.length; $i++) {
        new $Root(this, "env" + $i);
      }
    } else {
      new $Root(this, "Default");
    }
  }
}
new $App().synth();

```

