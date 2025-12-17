# 1. anchor开发
## 1.1 参考文档
[anchor 安装文档](https://www.anchor-lang.com/docs/installation)

这个文档指定了anchor依赖的一系列版本,但是这个文档里面的rustc 版本有点老,我编译的时候老出错.
会提示错误,大概是solana 对应有个toolchain,

我调试了一个可用的版本.

## 1.2 可用的版本配套

```shell
Installed Versions:
rustc: 1.90.0 (1159e78c4 2025-09-14)
solana: solana-cli 2.3.13 (src:5466f459; feat:2142755730, client:Agave)
yarn :1.22.22
node : v23.9.0
anchor : anchor-cli 0.32.1
Installation complete. Please restart your terminal to apply all changes.
```

# 2.安装 特定版本solana,anchor 
## 2.1 rust 版本

### 2.1.1 install rustup
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

### 2.1.2 install rustc 1.90.0

```shell
# 指定当前版本
rustup default 1.90.0
#检查版本
rustc --version 
```

## 2.2 solana 版本
### 2.2.1 安装最新solana和agave install solana and agave-install-init
```shell
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
```
### 2.2.2 export solana/agave path


```shell
echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc
```
### 2.2.3 指定solana版本
set solana version to 2.3.13
```shell
$ agave-install-init 2.3.13 
  ✨ 2.3.13 initialized

$ solana --version
solana-cli 2.3.13 (src:0b37b8fd; feat:3294202862, client:Agave)

```

## 2.3. 安装特定版本anchor 
### 2.3.1 install avm
 avm and set anchor version
```shell
cargo install --git https://github.com/solana-foundation/anchor avm --force
agave-install-init v2.2.3
avm use 0.32.1

```
### 2.3.2 指定anchor版本

```shell
avm use 0.32.1
anchor --verison
```
## 2.4  yarn install
```shell
npm install -g yarn

```
# 3 验证anchor项目

## 3.1 init project
```shell
anchor init prj1
```

```shell
anchor init --test-template rust prj2
```

## 3.2 编译project
```shell
cd prj1
anchor build
```




## 4 相关版本查看
```shell
rustc --version
solana --version
yarn -version
node --version
anchor --version

```

#### result:
```shell
rustc 1.90.0 (1159e78c4 2025-09-14)
solana-cli 2.3.13 (src:5466f459; feat:2142755730, client:Agave)
1.22.22
v23.9.0
anchor-cli 0.32.1

```

# 常见问题
常见问题：可能在执行agave-install-init 执行不完整，导致出现
```shell
ljl@ljl-hp:~/work/solana/prj1$ anchor build
error: not a directory: '/home/ljl/.local/share/solana/install/releases/2.3.13/solana-release/bin/platform-tools-sdk/sbf/dependencies/platform-tools/rust/lib'
```

可以检查是否有sbf目录

```shell
$ cd ~/.local/share/solana/install/releases/2.3.13/solana-release/bin/platform-tools-sdk/sbf/
$ bash env.sh. 
```
这个脚本会下载一个 platform-tools-linux-x86_64.tar.bz2
~/.local/share/solana/install/releases/2.3.13/solana-release/bin/platform-tools-sdk/sbf/dependencies/platform-tools

## 如果没有找到sbf 文件，可以从这里下载
目录来自 https://github.com/anza-xyz/agave/releases/download/v2.3.13/sbf-sdk.tar.bz2
下载的platform-tools 下载来自 https://github.com/anza-xyz/platform-tools/releases

安装完成查看toolchain
```shell
rustup show
```
能查看到solana 的toolchain