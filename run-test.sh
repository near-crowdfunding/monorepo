#!/bin/bash

echo ">> Source and map CONTRACT_NAME -> FT_CONTRACT_NAME"
source ft-neardev/dev-account.env
export FT_CONTRACT_NAME=$CONTRACT_NAME

echo ">> Source and map CONTRACT_NAME -> IDO_CONTRACT_NAME"
source ido-neardev/dev-account.env
export IDO_CONTRACT_NAME=$CONTRACT_NAME

# user name = trungtin2001.testnet
export USER_NAME=trungtin2001.testnet

echo ">> Running tests"

echo ">> Test 1: Storage deposit"
near call $FT_CONTRACT_NAME storage_deposit '{"account_id": "'$IDO_CONTRACT_NAME'", "registration_only": true}' --accountId $USER_NAME --amount 0.1

echo ">> Test 2: FT transfer call"
near call $FT_CONTRACT_NAME ft_transfer_call '{"receiver_id": "'$IDO_CONTRACT_NAME'", "amount": "10", "msg": "{\"kind\":0}"}' --depositYocto 1 --accountId $USER_NAME --gas 40000000000000
