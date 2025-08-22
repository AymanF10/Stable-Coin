import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import 'rpc-websockets/dist/lib/client';
import { Stablecoin } from "../target/types/stablecoin";
import { PythSolanaReceiver } from "@pythnetwork/pyth-solana-receiver";


describe("stablecoin", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = provider.wallet as anchor.Wallet;

  anchor.setProvider(provider);
  const program = anchor.workspace.stablecoin as Program<Stablecoin>;
  const pythSolanaReciever = new PythSolanaReceiver({connection, wallet});
  const SOL_PRICE_FEED_ID = "OxeredBb6fda2ceba41da15d4095dlda392a0d2f8ed0c6c7bcof4cfac8c280b56d";
  const solUsdPriceFeedAccount = pythSolanaReciever.getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID);

  const [collateralAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("collateral"), wallet.publicKey.toBuffer()], program.programId);


  it("Is initialized!", async () => {
    
    const tx = await program.methods
    .initializeConfig()
    .accounts({})
    .rpc({skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature", tx);
  });

   it("Deposit Collateral And Mint USDC", async () => {
    const amountCollateral = 1_00_000_000;
    const amountToMint = 1_00_000_000;

    const tx = await program.methods.depositCollateralAndMintTokens(
      new anchor.BN(amountCollateral),
      new anchor.BN(amountToMint)
    )
    .accounts({ priceUpdate: solUsdPriceFeedAccount})
    .rpc({ skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature", tx);
  });

  it("Redeem Collateral And Burn USDC", async () => {
    const amountCollateral = 500_000_000;
    const amountToBurn = 500_000_000;

    const tx = await program.methods
    .redeemCollateralAndBurnTokens(
      new anchor.BN(amountCollateral),
      new anchor.BN(amountToBurn)
    )
    .accounts({ priceUpdate: solUsdPriceFeedAccount})
    .rpc({ skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature", tx);
  });

  it("Update Config", async () => {
    const tx = await program.methods
    .updateConfig(new anchor.BN(100))
    .accounts({})
    .rpc({ skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature", tx);
  });

  it("Liquidation", async () => {
     const amountToBurn = 500_000_000
     const tx = await program.methods
    .liquidate(new anchor.BN(amountToBurn))
    .accounts({collateralAccount, priceUpdate: solUsdPriceFeedAccount})
    .rpc({ skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature", tx);
  });

    it("Update Config", async () => {
    const tx = await program.methods
    .updateConfig(new anchor.BN(2))
    .accounts({})
    .rpc({ skipPreflight: true, commitment: "confirmed"});

    console.log("Transaction signature", tx);
  });
});
