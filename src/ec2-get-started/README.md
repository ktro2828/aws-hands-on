# Hands-on #1: Launching an EC2 instance

- [Hands-on #1: Launching an EC2 instance](https://tomomano.github.io/learn-aws-by-coding/#sec_first_ec2)

![app_arch](./figs/app_architecture.png "App architecture")

| Instance     | vCPU | Memory(GiB) | Network bandwidth(Gbps) | Price per hour($) |
| :----------- | :--- | :---------- | :---------------------- | :---------------- |
| t2.macro     | 1    | 1           | -                       | 0.0116            |
| t2.small     | 1    | 2           | -                       | 0.023             |
| t2.medium    | 2    | 4           | -                       | 0.0464            |
| c5.24xlarge  | 96   | 192         | 25                      | 4.08              |
| c5n.18xlarge | 72   | 192         | 100                     | 3.888             |
| x1e.16xlarge | 64   | 1952        | 10                      | 13.344            |

## Destroy stack & ssh key pair

```shell
# Destroy stack
$ cdk destroy

# Delete ssh key pair
$ aws ec2 delete-key-pair --key-name $KEY_NAME
$ rm -f ~/.ssh/$KEY_NAME.pem
```
