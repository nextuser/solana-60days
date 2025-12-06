import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day24 } from "../target/types/day24";
import {program} from "@coral-xyz/anchor/dist/browser/src/native/system";
import {getTransactionAnalysis, airdropSol, confirmTransaction} from './util'
type PublicKey = anchor.web3.PublicKey;
const   Keypair = anchor.web3.Keypair;


anchor.setProvider(anchor.AnchorProvider.env());
const conn = anchor.getProvider().connection;

describe("day24_update_by_other",  () => {
    const program = anchor.workspace.day24 as Program<Day24>;
    let seeds = [Buffer.from("a")];
    const [myStorage,_bump] = anchor.web3.PublicKey.findProgramAddressSync(
    seeds,program.programId
    )

    const initStorage = async () => {
        let alice = Keypair.generate();
        await airdropSol(conn,alice.publicKey, 1e9);
        console.log("user:",alice.publicKey.toBase58())
        console.log("balance is :" + await anchor.getProvider().connection.getBalance(alice.publicKey));
        const tx = await program.methods.initialize().accounts({
            myStorage: myStorage,
            fren : alice.publicKey
        }).signers([alice]).rpc();
        console.log("Your transaction signature", tx);
        await  confirmTransaction(conn,tx);
        //await anchor.getProvider().connection.confirmTransaction(conn,conn,tx);
        console.log("fetch logs")
        const detail = await  getTransactionAnalysis(anchor.getProvider().connection, tx);
        console.log("tx detail",detail);
    };

    const updateStorage = async () => {
        let bob = Keypair.generate();
        await airdropSol(conn,bob.publicKey, 1e9);
        console.log("user:",bob.publicKey.toBase58())
        console.log("balance is :" + await anchor.getProvider().connection.getBalance(bob.publicKey));
        const tx = await program.methods.update(new anchor.BN(33)).accounts({
            myStorage: myStorage,
            fren : bob.publicKey
        }).signers([bob]).rpc();
        console.log("Your transaction signature", tx);
        await  confirmTransaction(conn,tx);
        //await anchor.getProvider().connection.confirmTransaction(conn,tx);
        console.log("fetch logs")
        const detail = await  getTransactionAnalysis(anchor.getProvider().connection, tx);
        console.log("tx detail",detail);
    }

    it("Is initialized!", initStorage);

    it("update!", updateStorage);
});



describe("day24_transfer",  () => {
    // Configure the client to use the local cluster.

    const alice = Keypair.generate();
    const bob = Keypair.generate();
    const program = anchor.workspace.day24 as Program<Day24>;
    const alice_seeds = [alice.publicKey.toBytes()];
    const bob_seeds = [bob.publicKey.toBytes()];
    const [alice_player, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
        alice_seeds, program.programId
    )
    const [bob_player, _bump2] = anchor.web3.PublicKey.findProgramAddressSync(
        bob_seeds, program.programId
    )
    console.log(`alice : ${alice.publicKey.toBase58()}, player: ${alice_player.toBase58()}`);
    console.log(`bob : ${bob.publicKey.toBase58()}, player: ${bob_player.toBase58()}`);
    console.log('alice_seeds', alice_seeds);

    const init_handler = async () => {
        await airdropSol(conn,bob.publicKey, 1e9);
        await airdropSol(conn,alice.publicKey, 1e9);
        console.log("alice:", alice.publicKey.toBase58())
        console.log("balance is :" + await anchor.getProvider().connection.getBalance(alice.publicKey));
        const tx = await program.methods.initPlayer().accounts({
            player: alice_player,
            signer: alice.publicKey,
            //system_program: anchor.web3.SystemProgram.programId
        }).signers([alice]).rpc();
        console.log("Your transaction signature", tx);
        await confirmTransaction(conn,tx);
        //await anchor.getProvider().connection.confirmTransaction(conn,tx);
        console.log("fetch logs")
        const detail = await getTransactionAnalysis(anchor.getProvider().connection, tx);
        console.log("tx detail", detail);


        const tx2 = await program.methods.initPlayer().accounts({
            player: bob_player,
            signer: bob.publicKey,
            //system_program: anchor.web3.SystemProgram.programId
        }).signers([bob]).rpc();
        console.log("Your transaction signature", tx);
        await confirmTransaction(conn,tx2);
        //await anchor.getProvider().connection.confirmTransaction(conn,tx);
        console.log("fetch logs")
        const detail2 = await getTransactionAnalysis(anchor.getProvider().connection, tx2);
        console.log("tx detail", detail2);
        // });
    }

    const transfer_handler = async () => {
        console.log("bob:", bob.publicKey.toBase58())
        console.log("balance is :" + await anchor.getProvider().connection.getBalance(bob.publicKey));
        const tx = await program.methods.transferPoints(new anchor.BN(33)).accounts({
            from: alice_player,
            to: bob_player,
            authority: alice.publicKey
        }).signers([alice]).rpc();
        console.log("Your transaction signature", tx);
        await confirmTransaction(conn,tx);
        //await anchor.getProvider().connection.confirmTransaction(conn,tx);
        console.log("fetch logs")
        const detail = await getTransactionAnalysis(anchor.getProvider().connection, tx);
        console.log("tx detail", detail);
    };
    it("init and transfer", async()=>{
        await  init_handler();
        await  transfer_handler();
    })
    // it("initial player!",init_handler);
    // it("transfer player!", transfer_handler);

});

