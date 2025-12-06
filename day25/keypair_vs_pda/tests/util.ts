import * as anchor from "@coral-xyz/anchor";
const { Connection, PublicKey } = anchor.web3;
type Connection = anchor.web3.Connection;
type PublicKey = anchor.web3.PublicKey;

/**
 * 交易分析返回值类型
 */
export interface TransactionDetails {
    /** 交易签名 */
    signature: string;
    /** 消耗的计算单元 */
    computeUnits: number | null;
    /** 交易手续费（lamports） */
    fee: number;
    /** 交易日志 */
    logs: string[] | null;
}

/**
 * Solana交易分析工具：获取交易的计算单元、手续费、日志等核心信息
 * @param connection Solana Connection实例
 * @param signature 交易签名（base58格式）
 * @param commitment 区块确认级别（默认confirmed）
 * @returns 交易分析结果 | null（交易不存在/未确认）
 * @throws 网络错误、签名格式错误等异常
 */
export async function getTransactionDetials(
    connection: anchor.web3.Connection,
    signature: string,
    commitment: anchor.web3.Commitment = "confirmed"
): Promise<TransactionDetails | null> {

    try {
        await confirmTransaction(connection, signature);

        // 获取交易详情
        const transaction: anchor.web3.VersionedTransactionResponse | null = await connection.getTransaction(
            signature,
            {
                commitment:'confirmed',
                maxSupportedTransactionVersion: undefined // 兼容所有交易版本
            }
        );
        // 解析交易元数据
        if (transaction?.meta) {
            return {
                signature,
                computeUnits: transaction.meta.computeUnitsConsumed ?? null, // 兼容未消耗CU的场景
                fee: transaction.meta.fee,
                logs: transaction.meta.logMessages ?? null
            };
        }
        return null;
    } catch (error) {
        console.error(`[交易分析失败] 签名${signature}：`, error);
        throw new error;
    }
}

export async function airdropSol(connection : Connection, publicKey:PublicKey, amount: number){
    let airdropTx = await connection.requestAirdrop(publicKey,amount);
    await confirmTransaction(connection, airdropTx);
}
export async function confirmTransaction(connection : Connection, tx:string) {
    const latestBlockHash = await connection.getLatestBlockhash();
    console.log("confirmTransaction:" + tx);
    await anchor.getProvider().connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    });
}

export async function printAccount(conn,pubKey:PublicKey,prompt){
    let accountInfo :anchor.web3.AccountInfo<Buffer> = await conn.getAccountInfo(pubKey)
    if(!accountInfo){
        console.log(prompt,"Acount of Address", null);
        return;
    }
    console.log(prompt,
        "Account of Address:",pubKey.toBase58(),
        "lamports",accountInfo.lamports,
        "space",accountInfo.space,
        "owner:",accountInfo.owner)
}