#!/usr/bin/env bash

# Create python environment
echo ">> Creating Python environment..."
python3 -m venv .env
. .env/bin/activate
python3 -m pip install -r requirements.txt

# Generate SSH key
echo ">> Generating ssh key..."
export KEY_NAME="MyKey"
aws ec2 create-key-pair --key-name ${KEY_NAME} --query 'KeyMaterial' --output text > ${KEY_NAME}.pem
mv ${KEY_NAME}.pem ~/.ssh/
chmod 400 ~/.ssh/${KEY_NAME}.pem

# Deploy EC2 instance
echo ">> Start deploying EC2 instance..."
cdk deploy -c key_name=${KEY_NAME}
