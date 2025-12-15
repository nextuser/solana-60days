# 简介
## 60 Days of Solana 
这是 一个60天学习solana教程的练习
[中文版本]（https://decert.me/tutorial/rareskills-solana-course/）
[english version] (https://rareskills.io/solana-tutorial)
# 开发环境搭建
- 参考 [anchor 编译项目相关配套版本(0.32.1)](./anchor-env-guide.md)
# 测试
## 测试环境
运行本地节点
```shell
solana-test-validator
```
## 相关脚本
- test-anchor.sh 
  如果事先已经启动validator ， 运行这个脚本
  - 脚本代码解释
```shell
unset ANCHOR_BUILD_DIR 
# anchor 测试，需要编译的程序存放在项目当前目录，不能放在其他目录
unset CARGO_TARGET_DIR
# 因为有时可能会把keypair json文件存放到其他项目目录，所以这里通过--provider.wallet指定keypair文件的路径
anchor test --provider.wallet `solana config get keypair | awk -F ':' '{print $2}' ` 
```
- test-validator.sh
  - 如果事先没有启动validator ， 运行这个脚本 （很多带initialize的测试，往往重启valiator不容易发生冲突）
