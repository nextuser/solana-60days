"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const anchor = __importStar(require("@coral-xyz/anchor"));
const web3_js_1 = require("@solana/web3.js");
const spl_token_1 = require("@solana/spl-token");
const chai_1 = require("chai");
async function confirmTransaction(connection, signature) {
    const Block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({ signature,
        blockhash: Block.blockhash,
        lastValidBlockHeight: Block.lastValidBlockHeight
    }, "confirmed");
}
async function airdrop(connection, pubkey) {
    let signature = await connection.requestAirdrop(pubkey, 10000000000);
    await confirmTransaction(connection, signature);
}
async function getTokenAmount(connection, ata) {
    return (await (0, spl_token_1.getAccount)(connection, ata, "confirmed")).amount;
}
const program = anchor.workspace.anchorEscrow;
async function getEscrowInfo(connection, mint_key, maker_key, seed) {
    const [escrow, bump] = anchor.web3.PublicKey.findProgramAddressSync([
        Buffer.from("escrow"),
        maker_key.toBuffer(),
        new anchor.BN(seed).toArrayLike(Buffer, "le", 8),
    ], program.programId);
    const vault = await (0, spl_token_1.getAssociatedTokenAddress)(mint_key, escrow, true, spl_token_1.TOKEN_PROGRAM_ID);
    return {
        seed,
        escrow,
        vault,
        bump
    };
}
describe("anchor_escrow", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const payer = anchor.getProvider().wallet.payer;
    const maker = anchor.web3.Keypair.generate();
    const taker = anchor.web3.Keypair.generate();
    const connection = anchor.getProvider().connection;
    let mintA = web3_js_1.Keypair.generate();
    let mintB = web3_js_1.Keypair.generate();
    let vault;
    let makerAtaA;
    let takerAtaB;
    let takerAtaA;
    let makerAtaB;
    const amount = 1000000n;
    const receive = 2000000n;
    // const [escrow, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
    //     [
    //       Buffer.from("escrow"),
    //       maker.publicKey.toBuffer(),
    //       new anchor.BN(seed).toArrayLike(Buffer, "le", 8),
    //     ],
    //     program.programId
    //   );
    const confirm_option = { commitment: "confirmed" };
    before(async () => {
        console.log("wait to create mint ");
        await (0, spl_token_1.createMint)(connection, payer, payer.publicKey, payer.publicKey, 6, mintA, confirm_option, spl_token_1.TOKEN_PROGRAM_ID);
        await (0, spl_token_1.createMint)(connection, payer, payer.publicKey, payer.publicKey, 6, mintB, confirm_option, spl_token_1.TOKEN_PROGRAM_ID);
        console.log("mint created");
        await airdrop(connection, maker.publicKey);
        await airdrop(connection, taker.publicKey);
        await airdrop(connection, payer.publicKey);
        console.log("air drop ok");
        let init_balance = await connection.getBalance(maker.publicKey);
        makerAtaA = await (0, spl_token_1.createAssociatedTokenAccount)(connection, payer, mintA.publicKey, maker.publicKey, confirm_option);
        makerAtaB = await (0, spl_token_1.createAssociatedTokenAccount)(connection, payer, mintB.publicKey, maker.publicKey, confirm_option);
        takerAtaB = await (0, spl_token_1.createAssociatedTokenAccount)(connection, payer, mintB.publicKey, taker.publicKey, confirm_option);
        takerAtaA = await (0, spl_token_1.createAssociatedTokenAccount)(connection, payer, mintA.publicKey, taker.publicKey, confirm_option);
        console.log("ata created for taker");
        console.log("ata created for maker");
        // let vault = await createAssociatedTokenAccount(connection,payer,mintA.publicKey,escrow,confirm_option);
        // console.log("ata created for vault");
    });
    it("test make an take", async () => {
        const seed = 1n;
        const escrowInfo = await getEscrowInfo(connection, mintA.publicKey, maker.publicKey, seed);
        await (0, spl_token_1.mintTo)(connection, payer, mintA.publicKey, makerAtaA, payer.publicKey, amount, [payer], confirm_option);
        console.log("mint tokenA to maker");
        await (0, spl_token_1.mintTo)(connection, payer, mintB.publicKey, takerAtaB, payer.publicKey, receive, [payer], confirm_option);
        console.log("mint tokenB to taker");
        const signature = await program.methods.make(new anchor.BN(seed), new anchor.BN(receive), new anchor.BN(amount))
            .accounts({
            maker: maker.publicKey,
            escrow: escrowInfo.escrow,
            mintA: mintA.publicKey,
            mintB: mintB.publicKey,
            makerAtaA: makerAtaA,
            vault: escrowInfo.vault,
            associatedTokenProgram: spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: spl_token_1.TOKEN_PROGRAM_ID,
        }).signers([maker]).rpc();
        console.log(103);
        (0, chai_1.expect)((await (0, spl_token_1.getAccount)(connection, makerAtaA)).amount).to.equal(0n);
        console.log(105);
        (0, chai_1.expect)((await (0, spl_token_1.getAccount)(connection, escrowInfo.vault)).amount).to.equal(amount);
        console.log(107);
        (0, chai_1.expect)((await (0, spl_token_1.getAccount)(connection, takerAtaB)).amount).to.equal(receive);
        await confirmTransaction(connection, signature);
        const meta = await connection.getParsedTransaction(signature, 'confirmed');
        console.log("----------------make transaction meta:", meta.meta);
        const signature2 = await program.methods.take().accounts({
            taker: taker.publicKey,
            maker: maker.publicKey,
            escrow: escrowInfo.escrow,
            mintA: mintA.publicKey,
            mintB: mintB.publicKey,
            takerAtaB: takerAtaB,
            takerAtaA: takerAtaA,
            makerAtaB: makerAtaB,
            vault: escrowInfo.vault,
            associatedTokenProgram: spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: spl_token_1.TOKEN_PROGRAM_ID,
        }).signers([taker]).rpc();
        await confirmTransaction(connection, signature2);
        const meta2 = await connection.getParsedTransaction(signature2, 'confirmed');
        console.log("----------------take transaction meta:", meta2.meta);
        await confirmTransaction(connection, signature2);
        console.log(128);
        (0, chai_1.expect)((await (0, spl_token_1.getAccount)(connection, makerAtaB)).amount).to.equal(receive);
        console.log(1);
        //vault distroyed
        //expect( await getTokenAmount(connection,vault)).to.equal(0n);
        //console.log(2);
        (0, chai_1.expect)(await getTokenAmount(connection, takerAtaB)).to.equal(0n);
        console.log(3);
        (0, chai_1.expect)(await getTokenAmount(connection, takerAtaA)).to.equal(amount);
        console.log(4);
        (0, chai_1.expect)(await getTokenAmount(connection, makerAtaA)).to.equal(0n);
    }); //end it
    it("test make an refund!", async () => {
        await (0, spl_token_1.mintTo)(connection, payer, mintA.publicKey, makerAtaA, payer.publicKey, amount, [payer], confirm_option);
        console.log("mint tokenA to maker");
        const seed = 2n;
        const escrowInfo = await getEscrowInfo(connection, mintA.publicKey, maker.publicKey, seed);
        (0, chai_1.expect)((await (0, spl_token_1.getAccount)(connection, makerAtaB)).amount).to.equal(receive);
        const signature = await program.methods.make(new anchor.BN(seed), new anchor.BN(receive), new anchor.BN(amount))
            .accounts({
            maker: maker.publicKey,
            escrow: escrowInfo.escrow,
            mintA: mintA.publicKey,
            mintB: mintB.publicKey,
            makerAtaA: makerAtaA,
            vault: escrowInfo.vault,
            associatedTokenProgram: spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: spl_token_1.TOKEN_PROGRAM_ID,
        }).signers([maker]).rpc();
        await confirmTransaction(connection, signature);
        const meta = await connection.getParsedTransaction(signature, 'confirmed');
        console.log("----------------make transaction meta:", meta.meta);
        console.log(206);
        (0, chai_1.expect)((await (0, spl_token_1.getAccount)(connection, makerAtaA)).amount).to.equal(0n);
        console.log(207);
        (0, chai_1.expect)((await (0, spl_token_1.getAccount)(connection, vault)).amount).to.equal(amount);
        const signature2 = await program.methods.refund().accounts({
            maker: maker.publicKey,
            escrow: escrowInfo.escrow,
            mintA: mintA.publicKey,
            vault: escrowInfo.vault,
            makerAtaA: makerAtaA,
            associatedTokenProgram: spl_token_1.ASSOCIATED_TOKEN_PROGRAM_ID,
            tokenProgram: spl_token_1.TOKEN_PROGRAM_ID,
        }).signers([maker]).rpc();
        await confirmTransaction(connection, signature2);
        const meta2 = await connection.getParsedTransaction(signature2, 'confirmed');
        console.log("----------------refund transaction meta:", meta2.meta);
        console.log(128);
        //vault distroyed
        //expect( await getTokenAmount(connection,vault)).to.equal(0n);
        //console.log(2);
        (0, chai_1.expect)(await getTokenAmount(connection, makerAtaA)).to.equal(amount);
    }); //end it
});
