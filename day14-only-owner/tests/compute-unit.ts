import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14OnlyOwner } from "../target/types/compute_unit

describe("day14-only-owner", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day14OnlyOwner as Program<Day14OnlyOwner>;
}