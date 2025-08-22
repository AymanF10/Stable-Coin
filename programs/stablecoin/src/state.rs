use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Collateral {
    // I store the owner's address here
    pub depositer: Pubkey,
    
    // This is where I keep the SOL collateral
    pub sol_account: Pubkey,
    
    // This tracks where the user's minted tokens go
    pub token_account: Pubkey,
    
    // I need to know exactly how much SOL is deposited as collateral
    pub lamport_balance: u64,
    
    // This helps me track how many stablecoins the user has minted
    pub amount_minted: u64,
    
    // These are for Solana's PDAs (Program Derived Addresses)
    pub bump: u8,
    pub bump_sol_account: u8,
    
    // I use this flag to know if the account has been set up
    pub is_initialized: bool,
}

// This is where I store all the protocol-wide settings
#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    // The admin who can update protocol parameters
    pub authority: Pubkey,
    
    // The address of my stablecoin token mint
    pub mint_account: Pubkey,
    
    // I use this to determine when positions can be liquidated (e.g. 50%)
    pub liquidation_threshold: u64,
    
    // Liquidators get this bonus to incentivize them (e.g. 10%)
    pub liquidation_bonus: u64,
    
    // The minimum health factor before liquidation (usually 1.0)
    pub min_health_factor: u64,
    
    // More PDA bumps for Solana's addressing
    pub bump: u8,
    pub bump_mint_account: u8,
}
