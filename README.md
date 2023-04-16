# aws-hands-on

- [AWSドキュメント](https://docs.aws.amazon.com/ja_jp/)
- [コードで学ぶAWS入門](https://tomomano.github.io/learn-aws-by-coding/)
- [Source code](https://github.com/tomomano/learn-aws-by-coding)

## Requirements
- Python >= 3.6
- Node.js >= 12.0
- AWS CLI == 2.0
- AWS CDK >= 1.1

## Install

Configure AWS credential with `aws-sdk`

```shell
$ aws configure --profile PROFILE_NAME
```

If `aws-sdk` has not been installed, create manually.

```shell
$ mkdir ~/.aws && cd ~/.aws
$ touch credentials config
```

Example of `credentials` and `config` is as shown below.
- `credentials`

```txt
[default]
aws_access_key_id = XXXXXXXXXX
aws_secret_access_key = XXXXXXXXXXX
```

- `config`

```txt
[default]
region = ap-northeast-1     # Edit to your region
output = json               # Edit to your desired output format
```

### With Docker

- Python == 3.10.8
- Node.js == 19.9.0
- AWS CLI >= 2.0
- AWS CDK == 1.100.0

```shell
$ docker build -t labc -f docker/Dockerfile .
```

### Without Docker

#### Node.js

```shell
$ sudo apt-get install -y nodejs

# Confirm the installation
$ node --version
v19.9.0

$ npm --version
9.6.3
```

#### AWS CLI

For the details,  see [Installing or updating the latest version of the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).

```shell
# For Linux x86-64
$ curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
$ unzip awscliv2.zip
$ sudo ./aws/install

# Confirm the installation
$ aws --version
```

#### AWS CDK

For the details, see [Getting started with the AWS CDK](https://docs.aws.amazon.com/cdk/v2/guide/getting_started.html).

```shell
# Install v.1.100
$ npm install -g aws-cdk@1.100

# Confirm the installation
$ cdk --version
```