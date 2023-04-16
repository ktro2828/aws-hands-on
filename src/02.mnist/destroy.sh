#!/bin/bash

# Destroy stack
cdk destroy

# Delete ssh key pairs
aws ec2 delete-key-pair --key-name ${KEY_NAME}
rm -f ~/.ssh/$KEY_NAME.pem

# Delete virtual env
deactivate
rm -rf .env