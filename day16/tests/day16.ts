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

function asciiArrayToString(asciiArr) {
  if (!Array.isArray(asciiArr)) return "";
  return String.fromCharCode(...asciiArr);
}

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
    const randValue = Math.floor(Math.pow(2,Math.random()*32));
    const mapInfoKey = new anchor.BN(randValue);
    
     it("init mapinfo", async()=>{
        let key = mapInfoKey;
        let value = "abc";
        console.log("use mapinfo key:", key);
        //u64, 8bytes
        let seeds = [key.toArrayLike(Buffer,'le',8)];


        const [mapInfoAddress,_bump3] = anchor.web3.PublicKey.findProgramAddressSync(
            seeds,program.programId);
        const tx = await program.methods.initMapInfo(key,value ).accounts({
            signer:anchor.getProvider().publicKey,
            info : mapInfoAddress,
            systemProgram: anchor.web3.SystemProgram.programId  // 添加这一行
        }).rpc();
        await conn.confirmTransaction(tx)
        console.log("init mapinfo  tx signature:", tx);
        const  mapinfo = await program.account.info.fetch(mapInfoAddress)
        //console.log("map data :",mapinfo.key, mapinfo.value);
        expect(mapinfo.key.toNumber()).to.equal(key.toNumber());
        expect(toString(mapinfo.value)).to.equal(value);

    })

    function toString(arr  :number[]):string{
        let str = String.fromCharCode(...arr);
        let index = str.indexOf("\x00");
        if(index != -1){
            str = str.substring(0,index);
        }
        index = str.indexOf("\u0000");
        if(index != -1){
            str = str.substring(0,index);
        }

        return str;
    }
    //console.log(toString([97, 98, 99, 33,0,0,0]));

    it("set mapinfo", async()=>{
        let key = mapInfoKey;
        let value = "def";
        //u64, 8bytes
        let seeds = [key.toArrayLike(Buffer,'le',8)];


        const [mapInfoAddress,_bump3] = anchor.web3.PublicKey.findProgramAddressSync(
            seeds,program.programId);
        const tx = await program.methods.setMapInfo(key,value ).accounts({
            info : mapInfoAddress,
        }).rpc();
        await conn.confirmTransaction(tx)
        console.log("init mapinfo  tx signature:", tx);
        const  mapinfo = await program.account.info.fetch(mapInfoAddress)
        console.log("map data :",mapinfo.key, mapinfo.value);
        expect(toString(mapinfo.value)).to.equal(value);
        expect(mapinfo.key.toNumber()).to.equal(key.toNumber());

    })
});