# Mini-Hand-on #0: AWS CLI

- S3に新しい保存領域(Bucket)を追加: バケット名=`my-bucket`
```shell
$ aws s3 mb s3://my-bucket --region ap-northeast-1
```

pythonを使う場合は、`boto3`を使う。

```python
import boto3

s3_client = boto3.client("s3", region_name="ap-northeast-1")
s3_client.create_bucket(Bucket="my-bucket")
```

- バケットにファイルをアップロード

```shell
$ echo "Hello world" > hello_world.txt
$ aws s3 cp hello_world.txt "s3://${bucketName}/hello_world.txt"
```

バケット内のファイル一覧を取得

```shell
$ aws s3 ls "s3://${bucketName}" --human-readable

20yy-mm-dd hh:mm:ss   13 Bytes hello_world.txt
```

使い終わったバケットの削除。空でないバケットは`--force`で強制削除する。

```shell
$ aws s3 rb "s3://${bucketName}" --force
```

- 新しいEC2インスタンスの起動
※この際、`--key-name`に指定するkey pair、`--security-group-ids`に指定されているsecurity groupが作成されている必要がある。

```shell
$ aws ec2 run-instances --image-id ami-xxxxxxxx --count 1 --instance-type t2.micro --key-name MyKeyPair --security-group-ids sg-903004f8 --subnet-id subnet-6e7f829e
```
pythonを使う場合

```python
import boto3

ec2_client = boto3.client("ec2")
ec2_client.run_instances(
    ImageId="ami-xxxxxxxxx",
    MinCount=1,
    MaxCount=1,
    KeyName="MyKeyPair",
    InstanceType="t2.micro",
    SecurityGroupIds=["sg-903004f8"],
    SubnetId="subnet-6e7f829e",
)
```

