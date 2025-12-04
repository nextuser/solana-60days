import {Connection}  from "@solana/web3.js";

export async function getTransactionAnalysis(connection: Connection,signature: string) {

    await  connection.confirmTransaction(signature);
    const transaction = await connection.getTransaction(signature, {
        commitment: 'confirmed',
        ///commitment:"finalized",
        //maxSupportedTransactionVersion: undefined,
    });
    console.log("transaction:" ,transaction, "signature :", signature);
    if (transaction?.meta) {
        return {
            signature,
            computeUnits: transaction.meta.computeUnitsConsumed,
            fee: transaction.meta.fee,
            logs: transaction.meta.logMessages
        };
    }
    return null;
}

