#!/bin/bash
# 这个脚本会启动本地valiator节点
# anchor 测试，需要编译的程序存放在项目当前目录，不能放在其他目录
unset ANCHOR_BUILD_DIR 
unset CARGO_TARGET_DIR
# 因为有时可能会把keypair json文件存放到其他项目目录，所以这里通过--provider.wallet指定keypair文件的路径
anchor test --provider.wallet `solana config get keypair | awk -F ':' '{print $2}' ` 
