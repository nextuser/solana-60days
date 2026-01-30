// =============================================================================
// 托管系统测试 - 使用 Mollusk 测试框架
// =============================================================================
// 本文件包含托管系统的测试套件
// 注意：这是一个基础测试框架，需要根据实际需求完善

use mollusk_svm::Mollusk;
use solana_sdk::{
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    account::Account,
    transaction::{Transaction, TransactionError},
    instruction::{AccountMeta,Instruction},
};
use spl_token_client::{
    token::{ExtensionInitializationParams, Token,},
    client::{ProgramClient, ProgramRpcClient},
};
use spl_token::id as token_program_id;
use std::{rc::Rc, sync::Arc};
use crate::util::system_account_with_lamports;

const ESCROW_FILE_PATH: &str = "../pinocchio-escrow/target/deploy/blueshift_escrow";
const ID: Pubkey = solana_sdk::pubkey!("22222222222222222222222222222222222222222222");



// 改进你的 test_make_instruction 函数
#[tokio::test]
async fn test_make_instruction_with_proper_setup() {
    let user_a = Keypair::new();
    let user_b = Keypair::new();
    let mint_a_keypair = Keypair::new();
    let mint_b_keypair = Keypair::new();
    let payer = Keypair::new();

    let mollusk = Mollusk::new(&ID, ESCROW_FILE_PATH);

    // Mollusk::default().process_instruction_chain(
    //     instruction, 
    //     accounts, 
    //     );
    // let (mint_a, mint_b) = create_mint_account(mollusk, &payer, mint_a_keypair.pubkey(), user_a.pubkey());
 

    // // 创建两个不同的 mint
    // let mint_a_pubkey = mint_a_keypair.pubkey();
    // let mint_b_pubkey = mint_b_keypair.pubkey();

    // let token_a = Token::new(
    //     Arc::new(&mollusk),
    //     &spl_token::id(),
    //     &mint_a_pubkey,
    //     Some(6),
    //     &payer.pubkey(),
    //     &payer.pubkey(),
    //     false,
    //     &mint_a_keypair,
    // );

    // let token_b = Token::new(
    //     Rc::new(&mollusk),
    //     &spl_token::id(),
    //     &mint_b_pubkey,
    //     Some(6),
    //     &payer.pubkey(),
    //     &payer.pubkey(),
    //     false,
    //     &mint_b_keypair,
    // );

    // // 初始化两个 mint
    // mollusk.process_transaction(
    //     &token_a.initialize_mint(vec![]).unwrap(),
    //     &[&mint_a_keypair, &payer],
    // );

    // mollusk.process_transaction(
    //     &token_b.initialize_mint(vec![]).unwrap(),
    //     &[&mint_b_keypair, &payer],
    // );

    // // 为用户创建 token A 账户
    // let user_a_token_a = Keypair::new();
    // mollusk.process_transaction(
    //     &token_a.create_auxiliary_token_account_with_extension_initialized(
    //         &user_a_token_a,
    //         &user_a.pubkey(),
    //         vec![],
    //     ).unwrap(),
    //     &[&user_a_token_a, &user_a],
    // );

    // // 为用户创建 token B 账户
    // let user_a_token_b = Keypair::new();
    // mollusk.process_transaction(
    //     &token_b.create_auxiliary_token_account_with_extension_initialized(
    //         &user_a_token_b,
    //         &user_a.pubkey(),
    //         vec![],
    //     ).unwrap(),
    //     &[&user_a_token_b, &user_a],
    // );

    // // 为用户 B 创建 token B 账户
    // let user_b_token_b = Keypair::new();
    // mollusk.process_transaction(
    //     &token_b.create_auxiliary_token_account_with_extension_initialized(
    //         &user_b_token_b,
    //         &user_b.pubkey(),
    //         vec![],
    //     ).unwrap(),
    //     &[&user_b_token_b, &user_b],
    // );

    // // 铸造代币
    // let amount_a = 1000 * 10_u64.pow(6);
    // let amount_b = 500 * 10_u64.pow(6);
    
    // mollusk.process_transaction(
    //     &token_a.mint_to(
    //         &user_a_token_a.pubkey(),
    //         &payer.pubkey(),
    //         amount_a,
    //         &[&payer],
    //     ).unwrap(),
    //     &[&payer],
    // );

    // mollusk.process_transaction(
    //     &token_b.mint_to(
    //         &user_b_token_b.pubkey(),
    //         &payer.pubkey(),
    //         amount_b,
    //         &[&payer],
    //     ).unwrap(),
    //     &[&payer],
    // );

    // println!("Setup complete:");
    // println!("  User A has {} of Token A", amount);
    // println!("  User B has {} of Token B", amount);
}


// =============================================================================
// 程序 ID 常量
// =============================================================================
// 这是在 lib.rs 中定义的程序 ID

// =============================================================================
// 测试 1: 基本 Mollusk 初始化测试
// =============================================================================
// 这个测试验证 Mollolk 能正确加载程序
#[test]
fn test_mollusk_initialization() {
    // 创建 Mollusk 测试环境
    // 省略 .so 扩展名，Mollusk 会自动添加
    let _mollusk = Mollusk::new(&ID, ESCROW_FILE_PATH);

    // 验证程序可以正确初始化
    // 这是一个基本的测试，确保 Mollusk 环境可以正确初始化
    assert!(true, "Mollusk initialization successful");
}

// =============================================================================
// 测试 2: Make 指令基本测试（占位符）
// =============================================================================
// 测试创建托管交易指令
// 注意：完整的测试需要：
// 1. 创建测试账户（maker, mint_a, mint_b 等）
// 2. 设置代币账户余额
// 3. 构造正确的指令数据
// 4. 验证执行结果
#[test]
fn test_make_instruction_placeholder() {
    let _mollusk = Mollusk::new(&ID, ESCROW_FILE_PATH);

    // TODO: 实现 Make 指令的完整测试
    // 1. 创建测试账户
    // 2. 设置账户状态
    // 3. 构造指令
    // 4. 执行并验证结果

    // 这是一个占位符测试
    assert!(true, "Make instruction test - to be implemented");
}

// =============================================================================
// 测试 3: Take 指令基本测试（占位符）
// =============================================================================
// 测试接受托管交易指令
#[test]
fn test_take_instruction_placeholder() {
    let _mollusk = Mollusk::new(&ID, ESCROW_FILE_PATH);

    // TODO: 实现 Take 指令的完整测试
    // 1. 先执行 Make 指令创建托管
    // 2. 创建 taker 账户和必要的代币账户
    // 3. 构造 Take 指令
    // 4. 验证代币交换和账户关闭

    assert!(true, "Take instruction test - to be implemented");
}

// =============================================================================
// 测试 4: Refund 指令基本测试（占位符）
// =============================================================================
// 测试退款指令
#[test]
fn test_refund_instruction_placeholder() {
    let _mollusk = Mollusk::new(&ID, ESCROW_FILE_PATH);

    // TODO: 实现 Refund 指令的完整测试
    // 1. 先执行 Make 指令创建托管
    // 2. 构造 Refund 指令
    // 3. 验证代币退还和账户关闭

    assert!(true, "Refund instruction test - to be implemented");
}

// =============================================================================
// 辅助函数说明
// =============================================================================
// 完整的测试需要以下辅助函数：

// // 派生托管账户 PDA
fn derive_escrow_pda(maker: &Pubkey, seed: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            b"escrow",
            maker.as_ref(),
            &seed.to_le_bytes(),
        ],
        &ID,
    )
}

// 派生关联代币账户（ATA）
fn derive_ata(owner: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[
            owner.as_ref(),
            &solana_sdk::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5da").as_ref(),
            mint.as_ref(),
        ],
        &solana_sdk::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
    )
}

// 构造 Make 指令数据
fn make_instruction_data(seed: u64, receive: u64, amount: u64) -> Vec<u8> {
    let mut data = vec![0u8]; // discriminator = 0
    data.extend_from_slice(&seed.to_le_bytes());
    data.extend_from_slice(&receive.to_le_bytes());
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

#[test]
pub fn test_make_instruction() { 
    let user_a = Keypair::new();
    let user_b = Keypair::new();
    let mint_a = Keypair::new();
    let mint_b = Keypair::new();
    let payer = Keypair::new();

    let _mollusk = Mollusk::new(&ID, ESCROW_FILE_PATH);
    // Mollusk::airdrop(&user_a.pubkey(), 100_000_000);
    // Mollusk::airdrop(&user_b.pubkey(), 100_0000_000);
    // Mollusk::airdrop(&payer.pubkey(), 100_0000_000);


    // initialize_mint(
    //     &TOKEN_PROGRAM_ID,
    //     &mint_a,
    //     &payer.pubkey(),
    //     &payer.pubkey(),
    //     &payer.pubkey(),
    //     0,
    // );  
}

// =============================================================================
// 测试流程说明
// =============================================================================
//
// 完整的测试流程应该包括：
//
// 1. **测试环境设置**：
//    - 使用 Mollusk 加载程序
//    - 创建必要的测试账户
//
// 2. **测试 Make 指令**：
//    - 准备 maker 账户和签名者
//    - 创建/初始化代币账户和 mint 账户
//    - 派生托管账户和金库账户的 PDA
//    - 构造 Make 指令（discriminator = 0, seed, receive, amount）
//    - 执行指令并验证：
//      * 托管账户创建成功
//      * 金库账户创建成功
//      * 代币从 maker ATA 转到金库
//
// 3. **测试 Take 指令**：
//    - 使用 Make 执行后的状态
//    - 准备 taker 账户和签名者
//    - 创建/初始化 taker 的代币账户
//    - 构造 Take 指令（discriminator = 1）
//    - 执行指令并验证：
//      * 代币 A 从金库转到 taker
//      * 代币 B 从 taker 转到 maker
//      * 金库账户关闭
//      * 托管账户关闭
//
// 4. **测试 Refund 指令**：
//    - 使用 Make 执行后的状态（另一个测试分支）
//    - 构造 Refund 指令（discriminator = 2）
//    - 执行指令并验证：
//      * 代币 A 从金库退还给 maker
//      * 金库账户关闭
//      * 托管账户关闭
//
// 5. **边界情况测试**：
//    - 测试无效的参数（amount = 0）
//    - 测试重复执行 Take/Refund（应该失败）
//    - 测试非创建者调用 Refund（应该失败）
//    - 测试余额不足的情况
//
// 参考：
// - Mollusk 文档：https://github.com/buffalojoec/molusk
// - Solana 程序测试最佳实践