from aws_cdk import core, aws_ec2 as ec2

import os


class CustomEC2(core.Stack):
    def __init__(self, scope: core.App, name: str, key_name: str, **kwargs) -> None:
        super().__init__(scope, name, **kwargs)

        # Create VPC (Virtual Private Cloud)
        vpc = ec2.Vpc(
            self, "CustomEC2-Vpc",
            max_azs=1,  # Availability Zone (AZ)
            cidr="10.10.0.0/23",  # Range of IPv4 in VPC. 10.10.0.0/23 = 10.10.0.0~10.10.1.255
            subnet_configuration=[
                ec2.SubnetConfiguration(
                    name="public",
                    subnet_type=ec2.SubnetType.PUBLIC,
                )
            ],  # Parameter about subnet in VPC
            nat_gateways=0,  # NOTE: If this is != 0, NAT Gateway charges will be incurred!!
        )

        # Security Group settings
        sg = ec2.SecurityGroup(
            self, "CustomEC2Vpc-Sg",
            vpc=vpc,
            allow_all_outbound=True,
        )
        sg.add_ingress_rule(
            peer=ec2.Peer.any_ipv4(),
            connection=ec2.Port.tcp(22),  # Allow access to port22 from all IPv4 address.
        )

        # Create EC2 instance
        host = ec2.Instance(
            self, "CustomEc2Instance",
            instance_type=ec2.InstanceType("t2.micro"),
            machine_image=ec2.MachineImage.latest_amazon_linux(),
            vpc=vpc,
            vpc_subnets=ec2.SubnetSelection(subnet_type=ec2.SubnetType.PUBLIC),
            security_group=sg,
            key_name=key_name,
        )

        # print the server address
        core.CfnOutput(self, "InstancePublicDnsName", value=host.instance_public_dns_name)
        core.CfnOutput(self, "InstancePublicIp", value=host.instance_public_ip)

app = core.App()
CustomEC2(
    app, "CustomEC2",
    key_name=app.node.try_get_context("key_name"),
    env={
        "region": os.environ["CDK_DEFAULT_REGION"],
        "account": os.environ["CDK_DEFAULT_ACCOUNT"],
    }
)

app.synth()