#!/bin/bash

cargo install dharitri-sc-meta

TARGET_DIR=$PWD/target

sc-meta all update --target-dir-all $TARGET_DIR --path ./contracts
