import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day13Event } from "../target/types/day13_event";

describe("day13_event", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.day13Event as Program<Day13Event>;


  it("Is initialized!", async () => {
      const listenMyEvent = program.addEventListener('myEvent', (event, slot) => {
          console.log(`slot ${slot},event value : ${event.value}`);
      })
      const listenSecondEvent = program.addEventListener("mySecondEvent",
          (event, slot) => {
              console.log(`slot : {slot}, value of second event : ${event.value}, message :${event.message}`)
          })
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    await new Promise((resolve)=>setTimeout(resolve,5000));
    program.removeEventListener(listenMyEvent);
    program.removeEventListener(listenSecondEvent);
    console.log("Your transaction signature", tx);
  });
});
