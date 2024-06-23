import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { BN } from "bn.js";
// import { expect } from '@jest/globals';
// npm install --save-dev @jest/globals @types/jest

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;
  const counterKp =new anchor.web3.Keypair();

  it("Is initialized!", async () => {
    // const count = new BN(0);
    const tx = await program.methods.crearContador(new BN(0)).accounts({
      contador: counterKp.publicKey,
      autoridad: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .signers([counterKp])
    .rpc();
    console.log("Your transaction signature", tx);

    const counter = await program.account.contador.fetch(counterKp.publicKey);
    console.log("Counter count is:", counter.numero.toNumber());
    
  });
  it("Aumenta el contador", async () => {
    // const counterKp =new anchor.web3.Keypair();
    // const counterBefore = program.account.contador.numero;
    const tx = await program.methods.aumentarContador(new BN(10)).accounts({
      contador: counterKp.publicKey,
      autoridad: provider.wallet.publicKey
    })
    .signers([counterKp])
    .rpc();
    console.log("Your transaction signature", tx);

    const counterAfter = await program.account.contador.fetch(counterKp.publicKey);
    console.log("Counter count now is:", counterAfter.numero.toNumber());

    // expect(counterAfter.numero.toNumber()).toEqual(new BN(10));
  })
});
