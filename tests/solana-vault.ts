import * as anchor from "@anchor-lang/core";
import { Program } from "@anchor-lang/core";
import { expect } from "chai";
import { BN } from "bn.js";
import {
  PublicKey,
  SystemProgram,
  LAMPORTS_PER_SOL,
  Keypair,
} from "@solana/web3.js";

import { SolanaVault } from "../target/types/solana_vault";

describe("solana-vault", () => {
  // 1. 连接本地 Solana 节点
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // 2. 获取 Anchor Program
  const program = anchor.workspace.solanaVault as Program<SolanaVault>;

  // 3. 获取当前钱包公钥
  const authority = provider.wallet.publicKey;

  // 4. 根据 seeds 推导 Vault PDA
  const [vaultPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("vault"),
      authority.toBuffer(),
    ],
    program.programId
  );

  // 5. 创建一个攻击者钱包
  const attacker = Keypair.generate();

  it("Initialize: should create a vault", async () => {
    await program.methods
      .initialize()
      .accounts({
        authority: authority,
      })
      .rpc();

    const vault = await program.account.vault.fetch(vaultPda);

    expect(vault.authority.toBase58()).to.equal(
      authority.toBase58()
    );

    expect(vault.totalDeposits.toString()).to.equal("0");

    console.log("Vault PDA:", vaultPda.toBase58());
    console.log("Initial deposits:", vault.totalDeposits.toString());
  });

  it("Deposit: should deposit 1 SOL", async () => {
    const amount = new BN(LAMPORTS_PER_SOL.toString());

    await program.methods
      .deposit(amount)
      .accountsStrict({
        vault: vaultPda,
        authority: authority,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const vault = await program.account.vault.fetch(vaultPda);

    expect(vault.totalDeposits.toString()).to.equal(
      amount.toString()
    );

    console.log("Deposited:", amount.toString(), "lamports");
    console.log("Total deposits:", vault.totalDeposits.toString());
  });

  it("Withdraw: should withdraw 0.4 SOL", async () => {
    const amount = new BN("400000000");

    await program.methods
      .withdraw(amount)
      .accountsStrict({
        vault: vaultPda,
        authority: authority,
      })
      .rpc();

    const vault = await program.account.vault.fetch(vaultPda);

    const expectedBalance = new BN("600000000");

    expect(vault.totalDeposits.toString()).to.equal(
      expectedBalance.toString()
    );

    console.log("Remaining deposits:", vault.totalDeposits.toString());
  });

  it("Security: attacker must not withdraw from another user's vault", async () => {
    const amount = new BN("100000000");

    let failed = false;

    try {
      await program.methods
        .withdraw(amount)
        .accounts({
          vault: vaultPda,
          authority: attacker.publicKey,
        })
        .signers([attacker])
        .rpc();
    } catch (error) {
      failed = true;
      console.log("Unauthorized withdrawal rejected:", String(error));
    }

    expect(failed).to.equal(true);

    // 确认攻击失败后，Vault 余额没有变化
    const vault = await program.account.vault.fetch(vaultPda);

    expect(vault.totalDeposits.toString()).to.equal("600000000");
  });
});