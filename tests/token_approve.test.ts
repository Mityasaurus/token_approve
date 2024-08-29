import { expect, describe, beforeAll, test } from "@jest/globals";
import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { TokenApprove } from "../target/types/token_approve";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction,
  getAssociatedTokenAddress,
  getMinimumBalanceForRentExemptMint,
  createInitializeMint2Instruction,
  MINT_SIZE,
} from "@solana/spl-token";

describe("token_approve", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const program = anchor.workspace.TokenApprove as Program<TokenApprove>;

  // Define test accounts and constants
  const alice = Keypair.generate();
  const bob = Keypair.generate();
  const usdcMint = Keypair.generate();
  const TOKEN_AMOUNT = 1_000_000;
  const OFFER_AMOUNT = 0.5;

  console.log(`Alice: ${alice.publicKey.toBase58()}`);
  console.log(`Bob: ${bob.publicKey.toBase58()}`);
  console.log(`usdcMint: ${usdcMint.publicKey.toBase58()}`);

  beforeAll(async () => {
    // Fund Alice and Bob
    await connection.requestAirdrop(alice.publicKey, 2 * LAMPORTS_PER_SOL);
    await connection.requestAirdrop(bob.publicKey, 2 * LAMPORTS_PER_SOL);

    // Create and initialize USDC token
    const minimumBalance = await getMinimumBalanceForRentExemptMint(connection);
    let tx = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: usdcMint.publicKey,
        lamports: minimumBalance,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeMint2Instruction(
        usdcMint.publicKey,
        6, // decimals
        alice.publicKey,
        null,
        TOKEN_PROGRAM_ID
      )
    );

    await provider.sendAndConfirm(tx, [usdcMint]);

    // Mint tokens to Alice
    const aliceTokenAccount = await getAssociatedTokenAddress(
      usdcMint.publicKey,
      alice.publicKey
    );
    const createTokenAccountIx = createAssociatedTokenAccountInstruction(
      alice.publicKey,
      aliceTokenAccount,
      alice.publicKey,
      usdcMint.publicKey
    );
    const mintToIx = createMintToInstruction(
      usdcMint.publicKey,
      aliceTokenAccount,
      alice.publicKey,
      TOKEN_AMOUNT
    );
    tx = new Transaction().add(createTokenAccountIx, mintToIx);
    await provider.sendAndConfirm(tx, [alice]);
  });

  test("create_offer and accept_offer", async () => {
    // Create Offer
    const offerAccount = Keypair.generate();

    await program.methods
      .createOffer(new BN(OFFER_AMOUNT))
      .accounts({
        initializer: alice.publicKey,
        initializerTokenAccount: await getAssociatedTokenAddress(
          usdcMint.publicKey,
          alice.publicKey
        ),
        offer: offerAccount.publicKey,
      })
      .signers([offerAccount, alice])
      .rpc();

    // Accept Offer
    const bobTokenAccount = await getAssociatedTokenAddress(
      usdcMint.publicKey,
      bob.publicKey
    );

    const createBobTokenAccountIx = createAssociatedTokenAccountInstruction(
      bob.publicKey,
      bobTokenAccount,
      bob.publicKey,
      usdcMint.publicKey
    );
    console.log("Created createBobTokenAccountIx");

    let tx = new Transaction().add(createBobTokenAccountIx);
    await provider.sendAndConfirm(tx, [bob]);
    console.log("Bob's token account created");

    await program.methods
      .acceptOffer()
      .accounts({
        receiver: bob.publicKey,
        initializerTokenAccount: await getAssociatedTokenAddress(
          usdcMint.publicKey,
          alice.publicKey
        ),
        receiverTokenAccount: bobTokenAccount,
        offer: offerAccount.publicKey,
      })
      .signers([bob])
      .rpc();
    console.log("Accepted offer");

    // Check token balances
    const aliceTokenAccountInfo = await connection.getTokenAccountBalance(
      await getAssociatedTokenAddress(usdcMint.publicKey, alice.publicKey)
    );

    const bobTokenAccountInfo = await connection.getTokenAccountBalance(
      bobTokenAccount
    );

    expect(aliceTokenAccountInfo.value.uiAmount).toBeLessThan(TOKEN_AMOUNT);
    expect(bobTokenAccountInfo.value.uiAmount).toEqual(OFFER_AMOUNT);
  });
});
