import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day16 } from "../target/types/day16";
const { expect } = require('chai');

describe("day16", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const conn = anchor.getProvider().connection;

  const program = anchor.workspace.day16 as Program<Day16>;
  //每次重新测试，可以修改一次seeds    这个seed "a" 和lib.rs 里面的 seed =[b"a"] 相对应
  const seeds = [Buffer.from("a")];
  const [myStorage,_bump ] = anchor.web3.PublicKey.findProgramAddressSync(
        seeds,program.programId
    );

    //最开始需要initialize
  // it("initialize myStorage!", async () => {
  //   // Add your test here.
  //
  //   console.log("the storage account address is ",myStorage.toBase58());
  //   const tx = await program.methods.initialize()
  //                       .accounts({
  //                        myStorage: myStorage,
  //                       })
  //   .rpc();
  //   console.log("Your transaction signature", tx);
  //   console.log("the storage account address is", myStorage.toBase58());
  // });

  it("set value to storage",async()=>{
      const tx = await program.methods.set(new anchor.BN(12))
          .accounts({myStorage:myStorage}).rpc();
      console.log("Your transaction signature", tx);
      let storageAccount = await conn.getAccountInfo(myStorage);
      console.log("account of myStorage:", storageAccount);
  })
    it("print storage value ", async()=>{
        const tx = await program.methods.printX()
            .accounts({myStorage:myStorage}).rpc();
        console.log("print tx signature:",tx);
    })

    //day17
    it("query storage!", async () => {
        let storageStruct = await program.account.myStorage.fetch(myStorage);
        console.log(`x is ${storageStruct.x}  y is ${storageStruct.y}  z is ${storageStruct.z}`);
    });



});

describe("day18_flag", async () => {
    anchor.setProvider(anchor.AnchorProvider.env());
    const conn = anchor.getProvider().connection;

    const program = anchor.workspace.day16 as Program<Day16>;
    const [setFlagAddr, _bump2] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("t")], program.programId);
    /** //只能init一次
    it("init flag", async()=>{
        const tx = await program.methods.initFlag(false).accounts({
            signer:anchor.getProvider().publicKey,
            trueOrFalse:setFlagAddr,
        }).rpc();
        await conn.confirmTransaction(tx)
        console.log("initFlag tx signature:", tx);
        const  flagData = await program.account.trueOrFalse.fetch(setFlagAddr)
        console.log("flag data :",flagData.flag);

    })
     **/

    it("setFlag", async()=>{
        const tx = await program.methods.setFlag(true).accounts({
            trueOrFalse:setFlagAddr,
            //trueOrFalse:setFlagAddr
        }).rpc();
        console.log("setFlag tx signature",tx)
        const  flagData = await program.account.trueOrFalse.fetch(setFlagAddr)
        console.log("flag data :",flagData.flag);

    })


    // it("init mapdata", async()=>{
    //     let key = new anchor.BN(35);
    //     //u64, 8bytes
    //     let seeds = [key.toArrayLike(Buffer,'le',8)];


    //     const [mapDataAddress,_bump3] = anchor.web3.PublicKey.findProgramAddressSync(
    //         seeds,program.programId);
    //     const tx = await program.methods.initMapData(key,new anchor.BN(11) ).accounts({
    //         signer:anchor.getProvider().publicKey,
    //         val : mapDataAddress,
    //     }).rpc();
    //     await conn.confirmTransaction(tx)
    //     console.log("init mapdata  tx signature:", tx);
    //     const  mapdata = await program.account.val.fetch(mapDataAddress)
    //     console.log("map data :",mapdata.key, mapdata.value);

    // })

    it("set mapdata", async()=>{
        let key = new anchor.BN(35);
        let value = new anchor.BN(16)
        //u64, 8bytes
        let seeds = [key.toArrayLike(Buffer,'le',8)];


        const [mapDataAddress,_bump3] = anchor.web3.PublicKey.findProgramAddressSync(
            seeds,program.programId);
        const tx = await program.methods.setMapData(key,value ).accounts({
            val : mapDataAddress,
        }).rpc();
        await conn.confirmTransaction(tx)
        console.log("init mapdata  tx signature:", tx);
        const  mapdata = await program.account.val.fetch(mapDataAddress)
        console.log("map data :",mapdata.key, mapdata.value);
        expect(mapdata.value.toNumber()).to.equal(value.toNumber());
        expect(mapdata.key.toNumber()).to.equal(key.toNumber());

    })
});