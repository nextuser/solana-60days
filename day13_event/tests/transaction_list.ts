import {PublicKey,Connection} from '@solana/web3.js';
import { execSync } from 'child_process';

function getDevnetUrl(){
    if(!process.env.HELIUS_API_KEY){
        console.log("export HELIUS_API_KEY first");
        process.exit(-1);
    }
    const solana_url = 'https://devnet.helius-rpc.com/?api-key=' + process.env.HELIUS_API_KEY;
    return solana_url;
}

function getLocalUrl(){
    return "http://localhost:8899";
}
    // Configure the client to use the local cluster.
//anchor.setProvider(anchor.AnchorProvider.env());

const wallet = ''
const solana_url = getLocalUrl();

export async function  getTransactions(connection :Connection,address: string,callback) {
    const pubKey = new PublicKey(address);
    let transactionList = await connection.getSignaturesForAddress(pubKey);
    console.log("tranaction list of ", pubKey.toBase58(), transactionList.length);
    let signatures = transactionList.map(tx => tx.signature);
    for  (let sig  of signatures){
        let  tx = await connection.getParsedTransaction(sig,{maxSupportedTransactionVersion:0});
        if(callback) callback(tx);
    }

}

function test(){
    const connection = new Connection(solana_url);
    const addr = execSync("solana address").toString().trim();
    getTransactions(connection,addr,(r)=>{
        console.log(r.slot, r.transaction.message.recentBlockhash,r.meta.computeUnitsConsumed,r.meta.fee)
    });
}

test();
