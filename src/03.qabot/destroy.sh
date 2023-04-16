#!/bin/bash

cdk destroy
aws ec2 delete-key-pair --key-name ${KEY_NAME}
rm -f ~/.ssh/$KEY_NAME.pem