from aws_cdk import core, aws_ec2 as ec2
import os

class EC2ForDL(core.Stack):
    def __init__(self, scope: core.App, name: str, key_name: str, **kwargs) -> None:
        super().__init__(scope, name, **kwargs)

        vpc = ec2.Vpc(
            self,  "EC2ForDL-Vpc",
            cidr="10.10.0.0/23",
            subnet_configuration=[ec2.SubnetConfiguration(name="public", subnet_type=ec2.SubnetType.PUBLIC)],
            nat_gateways=0,
        )

        sg = ec2.SecurityGroup(self, "EC2ForDL-Sg", vpc=vpc, allow_all_outbound=True)
        sg.add_ingress_rule(peer=ec2.Peer.any_ipv4(), connection=ec2.Port.tcp(22))

        host = ec2.Instance(
            self, "EC2ForDL-Instance",
            instance_type=ec2.InstanceType("g4dn.xlarge"),  # Specify instance type: g4dn.xlarge
            machine_image=ec2.MachineImage.generic_linux({
                "us-east-1": "ami-060f07284bb6f9faf",
                "ap-northeast-1": "ami-09c0c16fc46a29ed9"
            }),  # Specify AMI(Amazon Machine Image) that is pre-installed softwares for Machine Learning
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=ec2.SubnetType.PUBLIC),
            security_group=sg,
            key_name=key_name,
        )

        # print the server address
        core.CfnOutput(self, "InstancePublicDnsName", value=host.instance_public_dns_name)
        core.CfnOutput(self, "InstancePublicIp", value=host.instance_public_ip)

app = core.App()
EC2ForDL(
    app, "EC2ForDL", 
    key_name=app.node.try_get_context("key_name"),
    env={
        "region": os.environ["CDK_DEFAULT_REGION"],
        "account": os.environ["CDK_DEFAULT_ACCOUNT"],
    }
)

app.synth()