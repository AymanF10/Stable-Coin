use anchor_lang::prelude::*;

// I use these seed values to create deterministic PDAs (Program Derived Addresses)
pub const SEED_CONFIG_ACCOUNT: &[u8] = b"config";
pub const SEED_MINT_ACCOUNT: &[u8] = b"mint";
pub const SEED_COLLATERAL_ACCOUNT: &[u8] = b"collateral";
pub const SEED_SOL_ACCOUNT: &[u8] = b"sol";
pub const SEED_GOVERNANCE: &[u8] = b"governance";
pub const SEED_ORACLE_CONFIG: &[u8] = b"oracle";

// I'm using Pyth Network as my primary price oracle
#[constant]
pub const FEED_ID: &str = "OxeredBb6fda2ceba41da15d4095dlda392a0d2f8ed0c6c7bcof4cfac8c280b56d";

// I've added a backup oracle for redundancy and security
#[constant]
pub const BACKUP_FEED_ID: &str = "0x1111Bb6fda2ceba41da15d4095dlda392a0d2f8ed0c6c7bcof4cfac8c280aaaa";

// I don't want to use oracle prices that are too old (100 seconds max)
pub const MAXIMUM_AGE: u64 = 100;

// This helps me adjust the decimal precision from the oracle
pub const PRICE_FEED_DECIMAL_ADJUSTMENT: u128 = 10;

// I require at least 80% confidence in oracle prices
pub const ORACLE_CONFIDENCE_THRESHOLD: u8 = 80;

// If my two oracles disagree by more than 5%, something's wrong
pub const MAX_ORACLE_PRICE_DEVIATION: u8 = 5;

// My stablecoin has 9 decimals, matching SOL's precision
pub const MINT_DECIMALS: u8 = 9;

// I'll liquidate positions when collateral value falls below 50% of debt
pub const LIQUIDATION_THRESHOLD: u64 = 50;

// Liquidators get a 10% bonus to incentivize them
pub const LIQUIDATION_BONUS: u64 = 10;

// A health factor of 1 is my minimum threshold for safety
pub const MIN_HEALTH_FACTOR: u64 = 1;

// I warn users when their health factor drops below 2
pub const CRITICAL_HEALTH_FACTOR: u64 = 2;

// I don't want positions to be over-collateralized beyond 300%
pub const MAX_COLLATERAL_RATIO: u64 = 300;

// I require at least 150% collateralization for safety
pub const MIN_COLLATERAL_RATIO: u64 = 150;

// I apply a 5% discount to collateral value to account for volatility
pub const VOLATILITY_ADJUSTMENT: u64 = 5;

// I limit how many tokens can be minted per account for safety
pub const MAX_MINT_AMOUNT: u64 = 1_000_000_000;

// Users need at least 100,000 governance tokens to create proposals
pub const MIN_PROPOSAL_THRESHOLD: u64 = 100_000;

// Voting lasts for 24 hours (in seconds)
pub const VOTING_PERIOD: u64 = 86400;

// After a proposal passes, there's a 12-hour delay before execution
pub const EXECUTION_DELAY: u64 = 43200;

// At least 10% of tokens must vote for a proposal to pass
pub const QUORUM_THRESHOLD: u8 = 10;

// I charge a small 0.1% fee on minting (in basis points)
pub const MINT_FEE_BPS: u16 = 10;

// I charge a smaller 0.05% fee on burning
pub const BURN_FEE_BPS: u16 = 5;

// Liquidations have a 0.5% fee to generate protocol revenue
pub const LIQUIDATION_FEE_BPS: u16 = 50;

// Basis points divisor (10000 = 100%)
pub const BPS_DIVISOR: u16 = 10000;
