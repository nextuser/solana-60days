# cpi call signature with signer
[用PDA账号签名CPI调用](https://github.com/solana-foundation/developer-content/blob/main/content/guides/getstarted/how-to-cpi-with-signer.md)

日期	困难	标题	描述	标签	关键词
2024年4月24日 00:00:00 UTC
初学者
如何在 Solana 程序中使用 PDA 签名者进行 CPI
学习如何使用 Anchor 框架在 Solana 程序中通过 PDA 签名器实现跨程序调用 (CPI)。

本指南使用Anchor 框架来演示如何使用跨程序调用 (CPI)传输 SOL ，其中发送方是程序必须签名的 PDA。

这种场景的典型应用场景是代表用户 管理代币账户的程序 。例如，假设一个 DeFi 协议将用户资金汇集到一个账户中。该协议需要包含安全检查机制，以便自动处理提现请求。在这种情况下，这些汇集资金的控制权不在单个用户手中，而在于程序本身。这就需要使用 PDA作为协议代币账户的所有者，以编程方式签署提现请求。

下面列出了两种不同的、但功能相同的 Solana 实现，您在阅读或编写 Solana 程序时可能会遇到它们。这是 Solana Playground上的最终参考程序。

启动代码
这是 Solana Playground上的一个入门程序。该lib.rs文件包含以下仅有一条sol_transfer 指令的程序。

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("3455LkCS85a4aYmSeNbRrJsduNQfYRY82A7eCD3yQfyR");

#[program]
pub mod cpi {
    use super::*;

    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        let from_pubkey = ctx.accounts.pda_account.to_account_info();
        let to_pubkey = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let seed = to_pubkey.key();
        let bump_seed = ctx.bumps.pda_account;
        let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

        let cpi_context = CpiContext::new(
            program_id,
            Transfer {
                from: from_pubkey,
                to: to_pubkey,
            },
        )
        .with_signer(signer_seeds);

        transfer(cpi_context, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(
        mut,
        seeds = [b"pda", recipient.key().as_ref()],
        bump,
    )]
    pda_account: SystemAccount<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
该cpi.test.ts文件演示了如何调用自定义sol_transfer 指令，并记录了指向 SolanaFM 上交易详情的链接。

它展示了如何使用程序中指定的种子来推导PDA：

const [PDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("pda"), wallet.publicKey.toBuffer()],
  program.programId,
);
本示例的第一步是通过 Playground 钱包进行基本的 SOL 转账来为 PDA 账户充值。

it("Fund PDA with SOL", async () => {
  const transferInstruction = SystemProgram.transfer({
    fromPubkey: wallet.publicKey,
    toPubkey: PDA,
    lamports: transferAmount,
  });

  const transaction = new Transaction().add(transferInstruction);

  const transactionSignature = await sendAndConfirmTransaction(
    connection,
    transaction,
    [wallet.payer], // signer
  );

  console.log(
    `\nTransaction Signature:` +
      `https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`,
  );
});
PDA 充值 SOL 后，执行该sol_transfer指令。该指令会将 SOL 从 PDA 通过 CPI 转回wallet账户，转入系统程序，并由自定义程序进行“签名”。

it("SOL Transfer with PDA signer", async () => {
  const transactionSignature = await program.methods
    .solTransfer(new BN(transferAmount))
    .accounts({
      pdaAccount: PDA,
      recipient: wallet.publicKey,
    })
    .rpc();

  console.log(
    `\nTransaction Signature: https://solana.fm/tx/${transactionSignature}?cluster=devnet-solana`,
  );
});
交易详情将显示首先调用自定义程序（指令 1），然后调用系统程序（指令 1.1），从而成功完成 SOL 传输。

交易详情

您可以构建、部署和运行测试，以在 SolanaFM 浏览器中查看交易详情。

如何使用 Anchor 与签名者进行 CPI
在初始代码中，SolTransfer结构体指定了转账指令所需的账户。

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(
        mut,
        seeds = [b"pda", recipient.key().as_ref()],
        bump,
    )]
    pda_account: SystemAccount<'info>,
    #[account(mut)]
    recipient: SystemAccount<'info>,
    system_program: Program<'info, System>,
}
计算seeds地址时，需要pda_account包含硬编码字符串“pda”和账户地址recipient。这意味着每个账户的地址pda_account都是唯一的recipient。

用于生成 PDA 的 Javascript 等效代码包含在测试文件中。

const [PDA] = PublicKey.findProgramAddressSync(
  [Buffer.from("pda"), wallet.publicKey.toBuffer()],
  program.programId,
);
锚点 CpiContext
入门代码中包含的说明sol_transfer展示了使用 Anchor 框架构建 CPI 的典型方法。

这种方法涉及创建一个 CpiContext，其中包括program_id被调用指令所需的和帐户，然后是一个辅助函数（transfer）来调用特定指令。

pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
    let from_pubkey = ctx.accounts.pda_account.to_account_info();
    let to_pubkey = ctx.accounts.recipient.to_account_info();
    let program_id = ctx.accounts.system_program.to_account_info();

    let seed = to_pubkey.key();
    let bump_seed = ctx.bumps.pda_account;
    let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

    let cpi_context = CpiContext::new(
        program_id,
        Transfer {
            from: from_pubkey,
            to: to_pubkey,
        },
    )
    .with_signer(signer_seeds);

    transfer(cpi_context, amount)?;
    Ok(())
}
使用 PDA 签名时，可选种子和 Bump 种子包含在 使用cpi_context中。signer_seedswith_signer()

let seed = to_pubkey.key();
let bump_seed = ctx.bumps.pda_account;
let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

let cpi_context = CpiContext::new(
    program_id,
    Transfer {
        from: from_pubkey,
        to: to_pubkey,
    },
)
.with_signer(signer_seeds);
cpi_context然后amount将它们传递给transfer函数以执行 CPI。

transfer(cpi_context, amount)?;
当处理 CPI 时，Solana 运行时会验证所提供的种子和调用程序 ID 是否能派生出有效的 PDA。然后，该 PDA 会被添加为调用时的签名者。这种机制允许程序以编程方式对其程序 ID 派生的 PDA 进行签名。

使用 Crate Helper 调用
invoke_signed() 从本质上讲，上面的示例是对用于 system_instruction::transfer 构建指令的函数的封装。

下面的示例演示了如何使用该函数，通过PDA 签名的方法，invoke_signed()向系统程序的传输指令创建 CPI 。system_instruction::transfer

首先，将以下导入语句添加到文件顶部lib.rs：

use anchor_lang::solana_program::{program::invoke_signed, system_instruction};
接下来，sol_transfer对指令进行如下修改：

pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
    let from_pubkey = ctx.accounts.pda_account.to_account_info();
    let to_pubkey = ctx.accounts.recipient.to_account_info();
    let program_id = ctx.accounts.system_program.to_account_info();

    let seed = to_pubkey.key();
    let bump_seed = ctx.bumps.pda_account;
    let signer_seeds: &[&[&[u8]]] = &[&[b"pda", seed.as_ref(), &[bump_seed]]];

    let instruction =
        &system_instruction::transfer(&from_pubkey.key(), &to_pubkey.key(), amount);

    invoke_signed(instruction, &[from_pubkey, to_pubkey, program_id], signer_seeds)?;
    Ok(())
}
此实现与前面的示例在功能上等效。参数 signer_seeds被传递给invoke_signed函数。