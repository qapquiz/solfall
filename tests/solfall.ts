import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { createMint } from "@solana/spl-token";
import { Solfall } from "../target/types/solfall";

describe("solfall", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Solfall as Program<Solfall>;
  const userKeypair = anchor.web3.Keypair.generate();
  let tokenMint: anchor.web3.PublicKey;

  it("request airdrop", async () => {
    const airdropTx = await program.provider.connection.requestAirdrop(userKeypair.publicKey, anchor.web3.LAMPORTS_PER_SOL);
    await program.provider.connection.confirmTransaction(airdropTx);

    console.log("airdropTx:", airdropTx);
  });
  
  it("Create mint", async () => {
    tokenMint = await createMint(
      program.provider.connection,
      userKeypair,
      userKeypair.publicKey,
      null,
      9,
    );

    console.log("create mint:", tokenMint.toBase58());
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });

  it("Is DepositInitialized!", async () => {
    // @todo #2 create pda with seeds b"deposit_detail" + public key of depositor
    const [depositDetailPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from(anchor.utils.bytes.utf8.encode("deposit_detail")),
        userKeypair.publicKey.toBuffer(),
      ],
      program.programId
    );

    const tx = await program.methods
      .depositInitialize()
      .accounts({
        tokenMint: tokenMint,
        depositor: userKeypair.publicKey,
      })
      .signers([userKeypair])
      .rpc();

    console.log("DepositInitialize transaction signature", tx);
  });
});
