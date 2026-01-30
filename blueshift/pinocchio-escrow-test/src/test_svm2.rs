use anyhow::Result;
use mollusk_svm::{
    Mollusk,
    token::{
        CreateMintParams,
        CreateTokenAccountParams,
        TransferTokenParams,
        TokenDecimals,
    },
    simulator::SvmSimulator, // 核心：SVM 模拟器，提供本地离线模拟环境
};
use solana_sdk::{
    keypair::Keypair,
    signature::Signer,
    pubkey::Pubkey,
    lamports_to_sol,
};
use spl_associated_token_account::get_associated_token_address;

// 辅助函数：转换完整代币数量到最小单位（适配代币精度）
fn to_min_unit(full_amount: u64, decimals: u8) -> u64 {
    full_amount * 10u64.pow(decimals as u32)
}

#[tokio::test]
async fn test2() -> Result<()> {
    // ===================== 步骤 1：创建测试资源（模拟环境专用） =====================
    // 1. 创建 内存测试 Keypair（无需真实钱包文件，仅用于模拟签名，运行结束后销毁）
    let test_payer = Keypair::new();
    let payer_pubkey = test_payer.pubkey();
    println!("=== 模拟环境测试密钥对 ===");
    println!("测试钱包地址：{}", payer_pubkey);
    println!("测试钱包私钥（仅模拟有效）：{:?}", test_payer.to_bytes());

    // 2. 初始化 Mollusk SVM 模拟器（本地内存离线环境，核心！）
    // 无需连接 RPC 节点，所有操作均在内存中执行
    //let simulator = SvmSimulator::new();
    let mut mollusk = Mollusk::default();
    println!("\n=== Mollusk SVM 模拟环境初始化成功 ===");

    // 3. 给测试钱包分配 模拟 SOL（用于支付模拟租金/燃气费，无真实价值）
    // 模拟环境中账户初始余额为 0，需手动分配模拟 SOL 才能执行后续操作
    let mock_sol_amount = 10_000_000_000; // 10 个模拟 SOL（单位：lamports）
    mollusk.allocate_mock_sol(&payer_pubkey, mock_sol_amount).await?;
    let mock_balance = mollusk.get_mock_sol_balance(&payer_pubkey).await?;
    println!("测试钱包模拟 SOL 余额：{} lamports（{} SOL）", mock_balance, lamports_to_sol(mock_balance));

    // ===================== 步骤 2：模拟创建 Mint 账户（离线） =====================
    // 配置 Mint 模拟参数（与真实链上参数一致，仅在模拟环境生效）
    let mint_decimals = TokenDecimals::Nine; // 代币精度 9 位
    let full_supply = 1000u64; // 模拟铸币总量 1000 个完整代币
    let mint_params = CreateMintParams {
        payer: payer_pubkey,
        decimals: mint_decimals,
        total_supply: to_min_unit(full_supply, mint_decimals as u8), // 转换为最小单位
        name: "MockSvmDemoToken".to_string(),
        symbol: "MSDT".to_string(),
        uri: None, // 模拟元数据 URI（无需真实地址）
        mint_authority: Some(payer_pubkey),
        freeze_authority: Some(payer_pubkey),
    };

    // 模拟创建 Mint 账户（无链上交易，仅在内存中更新模拟状态）
    let (mint_address, default_token_account) = mollusk.create_mint(&test_payer, mint_params).await?;
    println!("\n=== 模拟 Mint 账户创建成功（离线）===");
    println!("Mint 地址（模拟唯一标识）：{}", mint_address);
    println!("自动创建的默认代币账户（模拟 ATA）：{}", default_token_account);

    // ===================== 步骤 3：模拟创建新的代币账户（离线） =====================
    // 计算模拟 ATA 账户地址（与真实 Solana ATA 计算逻辑一致）
    let new_token_account = get_associated_token_address(&payer_pubkey, &mint_address);
    let token_account_params = CreateTokenAccountParams {
        payer: payer_pubkey,
        owner: payer_pubkey,
        mint: mint_address,
        associated_token_account: new_token_account,
    };

    // 模拟创建代币账户（预留模拟租金，激活账户）
    mollusk.create_token_account(&test_payer, token_account_params).await?;
    println!("\n=== 模拟新代币账户创建成功（离线）===");
    println!("手动创建的模拟 ATA 账户：{}", new_token_account);

    // 验证模拟代币账户状态
    let account_info = mollusk.get_token_account(&new_token_account).await?;
    println!("模拟代币账户状态：已激活（数据大小：{} 字节）", account_info.data.len());

    // ===================== 步骤 4：模拟代币转账（离线） =====================
    // 配置模拟转账参数（仅同模拟 Mint 账户间可转账）
    let transfer_full_amount = 100u64; // 模拟转账 100 个完整代币
    let transfer_params = TransferTokenParams {
        mint: mint_address,
        from: default_token_account,
        to: new_token_account,
        amount: to_min_unit(transfer_full_amount, mint_decimals as u8), // 最小单位金额
        authority: payer_pubkey,
    };

    // 模拟代币转账（无链上签名，仅更新内存中的模拟账户余额）
    mollusk.transfer_token(&test_payer, transfer_params).await?;
    println!("\n=== 模拟代币转账成功（离线）===");
    println!("转账金额：{} 个完整 MSDT 代币", transfer_full_amount);
    println!("转出账户：{}", default_token_account);
    println!("转入账户：{}", new_token_account);

    // ===================== 步骤 5：验证模拟转账结果 =====================
    let default_account_balance = mollusk.get_token_balance(&default_token_account).await?;
    let new_account_balance = mollusk.get_token_balance(&new_token_account).await?;
    let decimals_pow = 10u64.pow(mint_decimals as u32) as f64;

    println!("\n=== 模拟转账结果验证 ===");
    println!("默认代币账户余额：{} MSDT", default_account_balance as f64 / decimals_pow);
    println!("新代币账户余额：{} MSDT", new_account_balance as f64 / decimals_pow);

    Ok(())
}

#[test]
fn test_call(){
    test2();
}
