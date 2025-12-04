查看两次anchor deploy之后可执行数据地址

第一次部署

ADmDHn5SffLaf6Y4MH838w3kNqx5zwXmLwUHhPtUsG2a => JB9o6mW5RZhVwrbJY5vzSkuXNbT5GkpqAk8N9mCvnJvM

```shell
ljl@ljl-lenovo:anchor-function-tutorial$ solana account ADmDHn5SffLaf6Y4MH838w3kNqx5zwXmLwUHhPtUsG2a

Public Key: ADmDHn5SffLaf6Y4MH838w3kNqx5zwXmLwUHhPtUsG2a
Balance: 0.00114144 SOL
Owner: BPFLoaderUpgradeab1e11111111111111111111111
Executable: true
Rent Epoch: 18446744073709551615
Length: 36 (0x24) bytes
0000:   02 00 00 00  ff 30 6e 1f  af a8 3a 41  8d f3 6c 93   .....0n...:A..l.
0010:   80 df 36 e1  12 bc 83 d7  61 4e 52 1d  ec 47 72 d7   ..6.....aNR..Gr.
0020:   70 cb 0c b2                                          p...

ljl@ljl-lenovo:anchor-function-tutorial$ echo -n 'JB9o6mW5RZhVwrbJY5vzSkuXNbT5GkpqAk8N9mCvnJvM' |base58 -d |xxd -p
ff306e1fafa83a418df36c9380df36e112bc83d7614e521dec4772d770cb
0cb2

```

# 查看地址
```bash
ljl@ljl-lenovo:day4$ solana program show ADmDHn5SffLaf6Y4MH838w3kNqx5zwXmLwUHhPtUsG2a

Program Id: ADmDHn5SffLaf6Y4MH838w3kNqx5zwXmLwUHhPtUsG2a
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: JB9o6mW5RZhVwrbJY5vzSkuXNbT5GkpqAk8N9mCvnJvM
Authority: JYLqLj2ztNMHUKXjqPthg9UiYWzkkBgGh6xJvg2v4hp
Last Deployed In Slot: 284916
Data Length: 191208 (0x2eae8) bytes
Balance: 1.33201176 SOL
```

# 修改代码，第二次部署
```bash
Program Id: ADmDHn5SffLaf6Y4MH838w3kNqx5zwXmLwUHhPtUsG2a
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: JB9o6mW5RZhVwrbJY5vzSkuXNbT5GkpqAk8N9mCvnJvM
Authority: JYLqLj2ztNMHUKXjqPthg9UiYWzkkBgGh6xJvg2v4hp
Last Deployed In Slot: 286567
Data Length: 191208 (0x2eae8) bytes
Balance: 1.33201176 SOL
```

发现ProgramData Address: JB9o6mW5RZhVwrbJY5vzSkuXNbT5GkpqAk8N9mCvnJvM 

Solana Program Account State 取值解析
根据您提供的数据，Program Account 的前4字节 02 00 00 00 表示账户状态：

状态值含义
值: 0x02 (little-endian格式，实际值为2)
含义: 表示这是一个可升级(upgradeable)的程序账户
Solana Program Account 状态类型
在Solana中，Program Account的状态通常定义为：
这个可以是program account 头部，也是program executable data account的头部

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UpgradeableLoaderState {
    Uninitialized,           // 0
    Buffer(BufferState),     // 1
    Program(ProgramState),   // 2
    ProgramData(ProgramDataState), // 3   program executable data account 采用这个值
}
```
状态值对应关系
0: Uninitialized - 未初始化状态
1: Buffer - 缓冲区状态
2: Program - 程序状态（您的情况）
3: ProgramData - 程序数据状态




# Program Authority 的存储位置
是的，您的理解是正确的：

Program Account (ADmDHn5SffLaf6Y4MH838w3kNqx5zwXmLwUHhPtUsG2a) 中只存储了指向 ProgramData Account 的地址
Program Authority 信息实际保存在 ProgramData Account (JB9o6mW5RZhVwrbJY5vzSkuXNbT5GkpqAk8N9mCvnJvM) 中
数据分布结构：
Program Account:

状态标识 (4 bytes)
ProgramData Account 地址 (32 bytes)
ProgramData Account:

状态信息 (包含 authority)
程序字节码数据
部署元数据
验证方式：
可以通过查看 ProgramData Account 的具体内容来确认 authority 信息：


# program data的定义源代码
[ProgramDataState](https://github.com/anza-xyz/solana-sdk/blob/master/loader-v3-interface/src/state.rs)

```rust
pub enum UpgradeableLoaderState {
    /// Account is not initialized.
    Uninitialized,
    /// A Buffer account.
    Buffer {
        /// Authority address
        authority_address: Option<Pubkey>,
        // The raw program data follows this serialized structure in the
        // account's data.
    },
    /// An Program account.
    Program {
        /// Address of the ProgramData account.
        programdata_address: Pubkey,
    },
    // A ProgramData account.
    ProgramData {
        /// Slot that the program was last modified.
        slot: u64,
        /// Address of the Program's upgrade authority.
        upgrade_authority_address: Option<Pubkey>,
        // The raw program data follows this serialized structure in the
        // account's data.
    },
}
```