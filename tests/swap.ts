import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Swap } from "../target/types/swap";
import {
  AddressLookupTableProgram,
  ComputeBudgetProgram,
  Connection,
  Keypair,
  PublicKey,
  Transaction,
} from "@solana/web3.js";
import {
  RaydiumLaunchpad,
  IDL as RaydiumLaunchpadIDL,
} from "../idl-types/raydium_launchpad";

describe("swap", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.swap as Program<Swap>;
  const raydium_program = new Program<RaydiumLaunchpad>(
    RaydiumLaunchpadIDL,
    program.provider
  );

  const platformConfig = new PublicKey(
    "BuM6KDpWiTcxvrpXywWFiw45R2RNH8WURdvqoTDV1BW4"
  );

  const baseTokenKeypair = new Keypair();
  const baseTokenPubkey = baseTokenKeypair.publicKey;

  const quoteTokenMint = new PublicKey(
    "USD1ttGY1N17NEEHLmELoaybftRBUSErhqYiQzvEmuB"
  );

  const tokenArgs = {
    name: "Launch Token",
    symbol: "LT",
    uri: "https://gateway.pinata.cloud/ipfs/bafkreigvfdqkujdxm6eyii4zxyxzfephwbxdfdighf5534w6cmot3a5uji",
  };

  const computeUnitIx = ComputeBudgetProgram.setComputeUnitLimit({
    units: 500_000,
  });
  async function createAndPopulateALT(
    connection: Connection,
    payer: Keypair,
    addresses: PublicKey[]
  ): Promise<PublicKey> {
    const slot = await connection.getSlot();

    const [createIx, lookupTableAddress] =
      AddressLookupTableProgram.createLookupTable({
        authority: payer.publicKey,
        payer: payer.publicKey,
        recentSlot: slot,
      });

    const extendIx = AddressLookupTableProgram.extendLookupTable({
      payer: payer.publicKey,
      authority: payer.publicKey,
      lookupTable: lookupTableAddress,
      addresses: addresses,
    });

    const tx = new Transaction().add(createIx, extendIx);
    await connection.sendTransaction(tx, [payer]);

    await new Promise((resolve) => setTimeout(resolve, 1000));

    return lookupTableAddress;
  }

  it("Is initialized!", async () => {
    // Add your test here.
    try {
      const txSig = await program.methods
        .initialize(tokenArgs)
        .accounts({
          quoteTokenMint: quoteTokenMint,
          baseTokenMint: baseTokenPubkey,
          platformConfig,
        })
        .preInstructions([computeUnitIx])
        .signers([baseTokenKeypair])
        .rpc();

      console.log("Transaction successful:", txSig);
    } catch (err: any) {
      if (err.logs) {
        console.error("Logs:", err.logs);
      } else {
        console.error("Full error:", err);
      }
    }

    // console.log("Your transaction signature", tx);
  });
});
