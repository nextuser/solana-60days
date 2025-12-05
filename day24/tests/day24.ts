import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day24 } from "../target/types/day24";
import {program} from "@coral-xyz/anchor/dist/browser/src/native/system";
type PublicKey = anchor.web3.PublicKey;
const   Keypair = anchor.web3.Keypair;
async function airdropSol(publicKey:PublicKey, amount: number){
    let airdropTx = await anchor.getProvider().connection.requestAirdrop(publicKey,amount);
    await confirmTransaction(airdropTx);
}
async function confirmTransaction(tx:string) {
    const latestBlockHash = await anchor.getProvider().connection.getLatestBlockhash();
    console.log("confirmTransaction:" + tx);
    await anchor.getProvider().connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: tx,
    });
}

describe("day24",  () => {
  // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.day24 as Program<Day24>;
    let seeds = [Buffer.from("a")];
    const [myStorage,_bump] = anchor.web3.PublicKey.findProgramAddressSync(
    seeds,program.programId
    )

  it("Is initialized!", async () => {
      let alice = Keypair.generate();
      await airdropSol(alice.publicKey, 1e9);
      console.log("user:",alice.publicKey.toBase58())
    console.log("balance is :" + await anchor.getProvider().connection.getBalance(alice.publicKey));
    const tx = await program.methods.initialize().accounts({
        myStorage: myStorage,
        program:anchor.web3.SystemProgram.programId,
        fren : alice.publicKey
    }).signers([alice]).rpc();
    console.log("Your transaction signature", tx);
    await  confirmTransaction(tx);
    //await anchor.getProvider().connection.confirmTransaction(tx);
    console.log("fetch logs")
    const detail = await  getTransactionAnalysis(anchor.getProvider().connection, tx);
    console.log("tx detail",detail);
  });

    it("update!", async () => {
        let bob = Keypair.generate();
        await airdropSol(bob.publicKey, 1e9);
        console.log("user:",bob.publicKey.toBase58())
        console.log("balance is :" + await anchor.getProvider().connection.getBalance(bob.publicKey));
        const tx = await program.methods.update(new anchor.BN(33)).accounts({
            myStorage: myStorage,
            program:anchor.web3.SystemProgram.programId,
            fren : bob.publicKey
        }).signers([bob]).rpc();
        console.log("Your transaction signature", tx);
        await  confirmTransaction(tx);
        //await anchor.getProvider().connection.confirmTransaction(tx);
        console.log("fetch logs")
        const detail = await  getTransactionAnalysis(anchor.getProvider().connection, tx);
        console.log("tx detail",detail);
    });

});

/**
 * 交易分析返回值类型
 */
export interface TransactionAnalysisResult {
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
export async function getTransactionAnalysis(
    connection: anchor.web3.Connection,
    signature: string,
    commitment: anchor.web3.Commitment = "confirmed"
): Promise<TransactionAnalysisResult | null> {

    try {
        await confirmTransaction(signature);

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