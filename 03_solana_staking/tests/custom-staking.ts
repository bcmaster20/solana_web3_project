/* eslint-disable */
import * as anchor from "@project-serum/anchor";
import {Program} from "@project-serum/anchor";
import {CustomStaking} from "../target/types/custom_staking";
import {
  PublicKey,
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction, Signer,
} from "@solana/web3.js";

const fs = require('fs');
const assert = require('assert');

import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  Token,
  TOKEN_PROGRAM_ID,
  u64,
} from '@solana/spl-token';

const utils = require('./utils');

let lock_time = new anchor.BN(3); //3 seconds
const STAKING_PDA_SEED = "staking";

const sleep = (ms) =>
  new Promise((resolve) => setTimeout(resolve, ms));

anchor.setProvider(anchor.AnchorProvider.env());
let provider = anchor.AnchorProvider.env();
const program = anchor.workspace.CustomStaking as Program<CustomStaking>;

let TestCase = 0;
describe("Custom-Staking Test Logic 11 Case", () => {
  let mintKey;
  let mintObject;
  let mintPubkey;

  //the program's vault for stored collateral against xToken minting
  let vaultPubkey;
  let vaultBump;

  let stakingPubkey;
  let stakingBump;
  let userStakingPubkey;
  let userStakingBump;
  let userStakingPubkey2;
  let userStakingBump2;

  it('1. Accounts Initialized!!!', async () => {
    //setup logging event listeners

    //this already exists in ecosystem
    //test step token hardcoded in program, mint authority is wallet for testing
    let rawdata = fs.readFileSync('tests/keys/step-teST1ieLrLdr4MJPZ7i8mgSCLQ7rTrPRjNnyFdHFaz9.json');
    let keyData = JSON.parse(rawdata);
    mintKey = anchor.web3.Keypair.fromSecretKey(new Uint8Array(keyData));
    mintObject = await utils.createMint(mintKey, provider, provider.wallet.publicKey, null, 9, TOKEN_PROGRAM_ID);
    mintPubkey = mintObject.publicKey;
    console.log("program.programId", program.programId.toBase58());

    [vaultPubkey, vaultBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [mintPubkey.toBuffer()],
        program.programId
      );

    [userStakingPubkey, userStakingBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [provider.wallet.publicKey.toBuffer()],
        program.programId
      );

    [stakingPubkey, stakingBump] =
      await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(anchor.utils.bytes.utf8.encode('staking'))],
        program.programId
      );

    await program.methods
      .initialize(lock_time)
      .accounts({
        tokenMint: mintPubkey,
        tokenVault: vaultPubkey,
        initializer: provider.wallet.publicKey,
        stakingAccount: stakingPubkey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      }).rpc();

  });

  let walletTokenAccount;

  it('2. Mint Test xTokens(100)', async () => {
    walletTokenAccount = await mintObject.createAssociatedTokenAccount(provider.wallet.publicKey);
    await utils.mintToAccount(provider, mintPubkey, walletTokenAccount, 100_000_000_000);
  });

  const stake = async (amount) => {
    await program.methods
      .stake(
        new anchor.BN(amount),
      )
      .accounts({
        tokenMint: mintPubkey,
        tokenFrom: walletTokenAccount,
        tokenFromAuthority: provider.wallet.publicKey,
        tokenVault: vaultPubkey,
        stakingAccount: stakingPubkey,
        userStakingAccount: userStakingPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  }

  const airdrop = async (amount) => {
      await utils.mintToAccount(provider, mintPubkey, vaultPubkey, amount * 1_000_000_000);
  }

  const withdraw = async () => {
    await program.methods
      .withdraw(vaultBump)
      .accounts({
        tokenMint: mintPubkey,
        xTokenFromAuthority: provider.wallet.publicKey,
        tokenVault: vaultPubkey,
        stakingAccount: stakingPubkey,
        userStakingAccount: userStakingPubkey,
        tokenTo: walletTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  }

  const unstake = async () => {
    await program.methods
      .unstake()
      .accounts({
        tokenMint: mintPubkey,
        xTokenFromAuthority: provider.wallet.publicKey,
        tokenVault: vaultPubkey,
        stakingAccount: stakingPubkey,
        userStakingAccount: userStakingPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  }

  const restake = async () => {
    await program.methods
      .restake()
      .accounts({
        tokenMint: mintPubkey,
        xTokenFromAuthority: provider.wallet.publicKey,
        tokenVault: vaultPubkey,
        stakingAccount: stakingPubkey,
        userStakingAccount: userStakingPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
  }

  it('3. Staking: Swap 5 Token, Token: 5 xToken: 95', async () => {
    await stake(5_000_000_000);

    assert.strictEqual(await getTokenBalance(walletTokenAccount), 95_000_000_000);
    assert.strictEqual(await getTokenBalance(vaultPubkey), 5_000_000_000);
  });

  it('4. Withdraw - Reject,  Result: Fail', async () => {
    await assert.rejects(
      async () => {
        await withdraw();
      }
    );
  });

  it('5. Airdrop 1 Token, Token: 6', async() => {
    const vaultAmount = await getTokenBalance(vaultPubkey);
    await airdrop(1);
    assert.strictEqual(await getTokenBalance(vaultPubkey), vaultAmount + 1_000_000_000);
  });

  it('6. Unstake', async () => {
    await unstake();
  });

  it('7. Restake', async () => {
    await restake();
  });

  it('8. Airdrop 2 Token, Token: 8', async() => {
    const vaultAmount = await getTokenBalance(vaultPubkey);
    await airdrop(2);
    assert.strictEqual(await getTokenBalance(vaultPubkey), vaultAmount + 2_000_000_000);
  });

  it('9. Unstake - Again', async () => {
    await unstake();
  });

  it('10. Airdrop 4 Token- Not Reward, Token: 12 xToken: 95', async() => {
    const vaultAmount = await getTokenBalance(vaultPubkey);
    await airdrop(4);
    assert.strictEqual(await getTokenBalance(vaultPubkey), vaultAmount + 4_000_000_000);
  });

  it('11. Withdraw Sleep LockTime+1(4) Token (4) xToken (103), ', async () => {
    await sleep(4000);
    await withdraw();
    assert.strictEqual(await getTokenBalance(walletTokenAccount), 103_000_000_000);
    assert.strictEqual(await getTokenBalance(vaultPubkey), 4_000_000_000);
  });
});

async function getTokenBalance(pubkey) {
  return parseInt(
    (await provider.connection.getTokenAccountBalance(pubkey)).value.amount
  );
}

// function createAssociatedTokenAccountInstruction(
//   associatedTokenAddress: PublicKey,
//   payer: PublicKey,
//   walletAddress: PublicKey,
//   splTokenMintAddress: PublicKey,
// ) {
//   const keys = [
//     {
//       pubkey: payer,
//       isSigner: true,
//       isWritable: true,
//     },
//     {
//       pubkey: associatedTokenAddress,
//       isSigner: false,
//       isWritable: true,
//     },
//     {
//       pubkey: walletAddress,
//       isSigner: false,
//       isWritable: false,
//     },
//     {
//       pubkey: splTokenMintAddress,
//       isSigner: false,
//       isWritable: false,
//     },
//     {
//       pubkey: SystemProgram.programId,
//       isSigner: false,
//       isWritable: false,
//     },
//     {
//       pubkey: TOKEN_PROGRAM_ID,
//       isSigner: false,
//       isWritable: false,
//     },
//     {
//       pubkey: SYSVAR_RENT_PUBKEY,
//       isSigner: false,
//       isWritable: false,
//     },
//   ];
//   return new TransactionInstruction({
//     keys,
//     programId: ASSOCIATED_TOKEN_PROGRAM_ID,
//     data: Buffer.from([]),
//   });
// }