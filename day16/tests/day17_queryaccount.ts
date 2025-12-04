import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day16 } from "../target/types/day16";
//import {describe,it} from 'mocha'
//
// describe("day17", () => {
//     // Configure the client to use the local cluster.
//     anchor.setProvider(anchor.AnchorProvider.env());
//
//     const program = anchor.workspace.day16 as Program<Day16>;
//     //每次重新测试，可以修改一次seeds
//     const seeds = [Buffer.from("a")];
//     const [myStorage,_bump ] = anchor.web3.PublicKey.findProgramAddressSync(
//         seeds,program.programId
//     );
//     const conn = anchor.getProvider().connection;
//     //最开始需要initialize
//     it("query storage!", async () => {
//         let storageStruct = await program.account.myStorage.fetch('myStorage');
//         console.log(`x is ${storageStruct.x}  y is ${storageStruct.y}  z is ${storageStruct.z}`);
//     });
// });