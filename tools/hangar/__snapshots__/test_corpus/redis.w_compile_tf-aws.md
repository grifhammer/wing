# [redis.w](../../../../examples/tests/valid/redis.w) | compile | tf-aws

## clients/$Inflight1.inflight.js
```js
module.exports = function({ r, r2 }) {
  class  $Inflight1 {
    constructor({  }) {
    }
    async handle()  {
      {
        const connection = (await r.rawClient());
        (await connection.set("wing","does redis"));
        const value = (await connection.get("wing"));
        {((cond) => {if (!cond) throw new Error(`assertion failed: '(value === "does redis")'`)})((value === "does redis"))};
        (await r2.set("wing","does redis again"));
        const value2 = (await r2.get("wing"));
        {((cond) => {if (!cond) throw new Error(`assertion failed: '(value2 === "does redis again")'`)})((value2 === "does redis again"))};
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
      "value": "[[\"root/Default/Default/test:test\",\"${aws_lambda_function.root_testtest_Handler_046C3415.arn}\"]]"
    }
  },
  "provider": {
    "aws": [
      {}
    ]
  },
  "resource": {
    "aws_eip": {
      "root_EIP_ED59AD4B": {
        "//": {
          "metadata": {
            "path": "root/Default/EIP",
            "uniqueId": "root_EIP_ED59AD4B"
          }
        }
      }
    },
    "aws_elasticache_cluster": {
      "root_r2_RedisCluster_EB1F5CD1": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/r2/RedisCluster",
            "uniqueId": "root_r2_RedisCluster_EB1F5CD1"
          }
        },
        "availability_zone": "${aws_subnet.root_PrivateSubnet_8A5D9DDE.availability_zone}",
        "cluster_id": "r2-c882797c",
        "engine": "redis",
        "engine_version": "6.2",
        "node_type": "cache.t4g.small",
        "num_cache_nodes": 1,
        "parameter_group_name": "default.redis6.x",
        "security_group_ids": [
          "${aws_security_group.root_r2_securityGroup_EC7D0A0D.id}"
        ],
        "subnet_group_name": "${aws_elasticache_subnet_group.root_r2_RedisSubnetGroup_873E9A00.name}"
      },
      "root_redisRedis_RedisCluster_2049056C": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/redis.Redis/RedisCluster",
            "uniqueId": "root_redisRedis_RedisCluster_2049056C"
          }
        },
        "availability_zone": "${aws_subnet.root_PrivateSubnet_8A5D9DDE.availability_zone}",
        "cluster_id": "redis-redis-c811048b",
        "engine": "redis",
        "engine_version": "6.2",
        "node_type": "cache.t4g.small",
        "num_cache_nodes": 1,
        "parameter_group_name": "default.redis6.x",
        "security_group_ids": [
          "${aws_security_group.root_redisRedis_securityGroup_C278C668.id}"
        ],
        "subnet_group_name": "${aws_elasticache_subnet_group.root_redisRedis_RedisSubnetGroup_6ED92583.name}"
      }
    },
    "aws_elasticache_subnet_group": {
      "root_r2_RedisSubnetGroup_873E9A00": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/r2/RedisSubnetGroup",
            "uniqueId": "root_r2_RedisSubnetGroup_873E9A00"
          }
        },
        "name": "r2-c882797c-subnetGroup",
        "subnet_ids": [
          "${aws_subnet.root_PrivateSubnet_8A5D9DDE.id}"
        ]
      },
      "root_redisRedis_RedisSubnetGroup_6ED92583": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/redis.Redis/RedisSubnetGroup",
            "uniqueId": "root_redisRedis_RedisSubnetGroup_6ED92583"
          }
        },
        "name": "redis-redis-c811048b-subnetGroup",
        "subnet_ids": [
          "${aws_subnet.root_PrivateSubnet_8A5D9DDE.id}"
        ]
      }
    },
    "aws_iam_role": {
      "root_testtest_Handler_IamRole_6C1728D1": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:test/Handler/IamRole",
            "uniqueId": "root_testtest_Handler_IamRole_6C1728D1"
          }
        },
        "assume_role_policy": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":\"sts:AssumeRole\",\"Principal\":{\"Service\":\"lambda.amazonaws.com\"},\"Effect\":\"Allow\"}]}"
      }
    },
    "aws_iam_role_policy": {
      "root_testtest_Handler_IamRolePolicy_65A1D8BE": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:test/Handler/IamRolePolicy",
            "uniqueId": "root_testtest_Handler_IamRolePolicy_65A1D8BE"
          }
        },
        "policy": "{\"Version\":\"2012-10-17\",\"Statement\":[{\"Action\":[\"elasticache:Describe*\"],\"Resource\":[\"${aws_elasticache_cluster.root_redisRedis_RedisCluster_2049056C.arn}\"],\"Effect\":\"Allow\"},{\"Action\":[\"elasticache:Describe*\"],\"Resource\":[\"${aws_elasticache_cluster.root_r2_RedisCluster_EB1F5CD1.arn}\"],\"Effect\":\"Allow\"},{\"Effect\":\"Allow\",\"Action\":[\"ec2:CreateNetworkInterface\",\"ec2:DescribeNetworkInterfaces\",\"ec2:DeleteNetworkInterface\",\"ec2:DescribeSubnets\",\"ec2:DescribeSecurityGroups\"],\"Resource\":\"*\"},{\"Effect\":\"Allow\",\"Action\":[\"ec2:CreateNetworkInterface\",\"ec2:DescribeNetworkInterfaces\",\"ec2:DeleteNetworkInterface\",\"ec2:DescribeSubnets\",\"ec2:DescribeSecurityGroups\"],\"Resource\":\"*\"}]}",
        "role": "${aws_iam_role.root_testtest_Handler_IamRole_6C1728D1.name}"
      }
    },
    "aws_iam_role_policy_attachment": {
      "root_testtest_Handler_IamRolePolicyAttachment_3716AC26": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:test/Handler/IamRolePolicyAttachment",
            "uniqueId": "root_testtest_Handler_IamRolePolicyAttachment_3716AC26"
          }
        },
        "policy_arn": "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole",
        "role": "${aws_iam_role.root_testtest_Handler_IamRole_6C1728D1.name}"
      }
    },
    "aws_internet_gateway": {
      "root_InternetGateway_8BBAF39E": {
        "//": {
          "metadata": {
            "path": "root/Default/InternetGateway",
            "uniqueId": "root_InternetGateway_8BBAF39E"
          }
        },
        "tags": {
          "Name": "Default-c82bf964-internet-gateway"
        },
        "vpc_id": "${aws_vpc.root_VPC_15726F37.id}"
      }
    },
    "aws_lambda_function": {
      "root_testtest_Handler_046C3415": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:test/Handler/Default",
            "uniqueId": "root_testtest_Handler_046C3415"
          }
        },
        "environment": {
          "variables": {
            "REDIS_CLUSTER_ID_30c8c4ae": "${aws_elasticache_cluster.root_r2_RedisCluster_EB1F5CD1.cluster_id}",
            "REDIS_CLUSTER_ID_5c75c5cf": "${aws_elasticache_cluster.root_redisRedis_RedisCluster_2049056C.cluster_id}",
            "WING_FUNCTION_NAME": "Handler-c8f4f2a1"
          }
        },
        "function_name": "Handler-c8f4f2a1",
        "handler": "index.handler",
        "publish": true,
        "role": "${aws_iam_role.root_testtest_Handler_IamRole_6C1728D1.arn}",
        "runtime": "nodejs18.x",
        "s3_bucket": "${aws_s3_bucket.root_Code_02F3C603.bucket}",
        "s3_key": "${aws_s3_object.root_testtest_Handler_S3Object_71CD07AC.key}",
        "timeout": 30,
        "vpc_config": {
          "security_group_ids": [
            "${aws_security_group.root_redisRedis_securityGroup_C278C668.id}",
            "${aws_security_group.root_r2_securityGroup_EC7D0A0D.id}"
          ],
          "subnet_ids": [
            "${aws_subnet.root_PrivateSubnet_8A5D9DDE.id}",
            "${aws_subnet.root_PrivateSubnet_8A5D9DDE.id}"
          ]
        }
      }
    },
    "aws_nat_gateway": {
      "root_NATGateway_23214D12": {
        "//": {
          "metadata": {
            "path": "root/Default/NATGateway",
            "uniqueId": "root_NATGateway_23214D12"
          }
        },
        "allocation_id": "${aws_eip.root_EIP_ED59AD4B.id}",
        "subnet_id": "${aws_subnet.root_PublicSubnet_C9666BAA.id}",
        "tags": {
          "Name": "Default-c82bf964-nat-gateway"
        }
      }
    },
    "aws_route_table": {
      "root_PrivateRouteTable_E7613E5F": {
        "//": {
          "metadata": {
            "path": "root/Default/PrivateRouteTable",
            "uniqueId": "root_PrivateRouteTable_E7613E5F"
          }
        },
        "route": [
          {
            "carrier_gateway_id": null,
            "cidr_block": "0.0.0.0/0",
            "core_network_arn": null,
            "destination_prefix_list_id": null,
            "egress_only_gateway_id": null,
            "gateway_id": null,
            "instance_id": null,
            "ipv6_cidr_block": null,
            "local_gateway_id": null,
            "nat_gateway_id": "${aws_nat_gateway.root_NATGateway_23214D12.id}",
            "network_interface_id": null,
            "transit_gateway_id": null,
            "vpc_endpoint_id": null,
            "vpc_peering_connection_id": null
          }
        ],
        "tags": {
          "Name": "Default-c82bf964-private-route-table-1"
        },
        "vpc_id": "${aws_vpc.root_VPC_15726F37.id}"
      },
      "root_PublicRouteTable_36451040": {
        "//": {
          "metadata": {
            "path": "root/Default/PublicRouteTable",
            "uniqueId": "root_PublicRouteTable_36451040"
          }
        },
        "route": [
          {
            "carrier_gateway_id": null,
            "cidr_block": "0.0.0.0/0",
            "core_network_arn": null,
            "destination_prefix_list_id": null,
            "egress_only_gateway_id": null,
            "gateway_id": "${aws_internet_gateway.root_InternetGateway_8BBAF39E.id}",
            "instance_id": null,
            "ipv6_cidr_block": null,
            "local_gateway_id": null,
            "nat_gateway_id": null,
            "network_interface_id": null,
            "transit_gateway_id": null,
            "vpc_endpoint_id": null,
            "vpc_peering_connection_id": null
          }
        ],
        "tags": {
          "Name": "Default-c82bf964-public-route-table-1"
        },
        "vpc_id": "${aws_vpc.root_VPC_15726F37.id}"
      }
    },
    "aws_route_table_association": {
      "root_PrivateRouteTableAssociation_A90EF7F6": {
        "//": {
          "metadata": {
            "path": "root/Default/PrivateRouteTableAssociation",
            "uniqueId": "root_PrivateRouteTableAssociation_A90EF7F6"
          }
        },
        "route_table_id": "${aws_route_table.root_PrivateRouteTable_E7613E5F.id}",
        "subnet_id": "${aws_subnet.root_PrivateSubnet_8A5D9DDE.id}"
      },
      "root_PublicRouteTableAssociation_22B41679": {
        "//": {
          "metadata": {
            "path": "root/Default/PublicRouteTableAssociation",
            "uniqueId": "root_PublicRouteTableAssociation_22B41679"
          }
        },
        "route_table_id": "${aws_route_table.root_PublicRouteTable_36451040.id}",
        "subnet_id": "${aws_subnet.root_PublicSubnet_C9666BAA.id}"
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
      "root_testtest_Handler_S3Object_71CD07AC": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/test:test/Handler/S3Object",
            "uniqueId": "root_testtest_Handler_S3Object_71CD07AC"
          }
        },
        "bucket": "${aws_s3_bucket.root_Code_02F3C603.bucket}",
        "key": "<ASSET_KEY>",
        "source": "<ASSET_SOURCE>"
      }
    },
    "aws_security_group": {
      "root_r2_securityGroup_EC7D0A0D": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/r2/securityGroup",
            "uniqueId": "root_r2_securityGroup_EC7D0A0D"
          }
        },
        "egress": [
          {
            "cidr_blocks": [
              "0.0.0.0/0"
            ],
            "description": null,
            "from_port": 0,
            "ipv6_cidr_blocks": null,
            "prefix_list_ids": null,
            "protocol": "-1",
            "security_groups": null,
            "self": null,
            "to_port": 0
          }
        ],
        "ingress": [
          {
            "cidr_blocks": [
              "${aws_subnet.root_PrivateSubnet_8A5D9DDE.cidr_block}"
            ],
            "description": null,
            "from_port": 6379,
            "ipv6_cidr_blocks": null,
            "prefix_list_ids": null,
            "protocol": "tcp",
            "security_groups": null,
            "self": true,
            "to_port": 6379
          }
        ],
        "name": "30c8c4ae-securityGroup",
        "vpc_id": "${aws_vpc.root_VPC_15726F37.id}"
      },
      "root_redisRedis_securityGroup_C278C668": {
        "//": {
          "metadata": {
            "path": "root/Default/Default/redis.Redis/securityGroup",
            "uniqueId": "root_redisRedis_securityGroup_C278C668"
          }
        },
        "egress": [
          {
            "cidr_blocks": [
              "0.0.0.0/0"
            ],
            "description": null,
            "from_port": 0,
            "ipv6_cidr_blocks": null,
            "prefix_list_ids": null,
            "protocol": "-1",
            "security_groups": null,
            "self": null,
            "to_port": 0
          }
        ],
        "ingress": [
          {
            "cidr_blocks": [
              "${aws_subnet.root_PrivateSubnet_8A5D9DDE.cidr_block}"
            ],
            "description": null,
            "from_port": 6379,
            "ipv6_cidr_blocks": null,
            "prefix_list_ids": null,
            "protocol": "tcp",
            "security_groups": null,
            "self": true,
            "to_port": 6379
          }
        ],
        "name": "5c75c5cf-securityGroup",
        "vpc_id": "${aws_vpc.root_VPC_15726F37.id}"
      }
    },
    "aws_subnet": {
      "root_PrivateSubnet_8A5D9DDE": {
        "//": {
          "metadata": {
            "path": "root/Default/PrivateSubnet",
            "uniqueId": "root_PrivateSubnet_8A5D9DDE"
          }
        },
        "cidr_block": "10.0.4.0/22",
        "tags": {
          "Name": "Default-c82bf964-private-subnet-1"
        },
        "vpc_id": "${aws_vpc.root_VPC_15726F37.id}"
      },
      "root_PublicSubnet_C9666BAA": {
        "//": {
          "metadata": {
            "path": "root/Default/PublicSubnet",
            "uniqueId": "root_PublicSubnet_C9666BAA"
          }
        },
        "cidr_block": "10.0.0.0/24",
        "tags": {
          "Name": "Default-c82bf964-public-subnet-1"
        },
        "vpc_id": "${aws_vpc.root_VPC_15726F37.id}"
      }
    },
    "aws_vpc": {
      "root_VPC_15726F37": {
        "//": {
          "metadata": {
            "path": "root/Default/VPC",
            "uniqueId": "root_VPC_15726F37"
          }
        },
        "cidr_block": "10.0.0.0/16",
        "enable_dns_hostnames": true,
        "enable_dns_support": true,
        "tags": {
          "Name": "Default-c82bf964-vpc"
        }
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
const redis = require('@winglang/sdk').redis;
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
        const r_client = context._lift(r);
        const r2_client = context._lift(r2);
        return $stdlib.core.NodeJsCode.fromInline(`
          require("${self_client_path}")({
            r: ${r_client},
            r2: ${r2_client},
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
          this._registerBindObject(r, host, ["rawClient"]);
          this._registerBindObject(r2, host, ["get", "set"]);
        }
        super._registerBind(host, ops);
      }
    }
    const r = this.node.root.newAbstract("@winglang/sdk.redis.Redis",this,"redis.Redis");
    const r2 = this.node.root.newAbstract("@winglang/sdk.redis.Redis",this,"r2");
    this.node.root.new("@winglang/sdk.cloud.Test",cloud.Test,this,"test:test",new $Inflight1(this,"$Inflight1"));
  }
}
class $App extends $AppBase {
  constructor() {
    super({ outdir: $outdir, name: "redis", plugins: $plugins, isTestEnvironment: $wing_is_test });
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

