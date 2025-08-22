use anchor_lang::prelude::*;

// I've defined all the possible errors my stablecoin program might encounter
#[error_code]
pub enum CustomError {
    // I need valid price data to safely operate the protocol
    #[msg("Invalid Price: Oracle price feed returned an invalid or stale price")]
    InvalidPrice,

    // I won't allow users to mint too many tokens against their collateral
    #[msg("Below Minimum Health Factor: Position is undercollateralized")]
    BelowMinHealthFactor,

    // I only allow liquidation when positions are actually at risk
    #[msg("Cannot liquidate a healthy account: Health factor is above minimum threshold")]
    AboveMinHealthFactor,
    
    // I need high confidence in oracle prices for safety
    #[msg("Oracle price confidence is below required threshold")]
    LowOracleConfidence,
    
    // If my oracles disagree too much, something might be wrong
    #[msg("Oracle prices deviate beyond acceptable threshold")]
    OraclePriceDeviation,
    
    // I discourage excessive over-collateralization
    #[msg("Collateral ratio exceeds maximum allowed")]
    ExcessiveCollateralization,
    
    // I require a minimum collateralization ratio for safety
    #[msg("Collateral ratio below minimum required")]
    InsufficientCollateralization,
    
    // I limit the maximum amount users can mint for safety
    #[msg("Mint amount exceeds maximum allowed per account")]
    ExcessiveMintAmount,
    
    // I warn users when their positions are approaching danger
    #[msg("Operation would result in critical health factor")]
    CriticalHealthFactor,
    
    // I require a minimum token balance to create governance proposals
    #[msg("Insufficient governance token balance for proposal")]
    InsufficientGovernanceBalance,
    
    // I enforce strict voting periods for governance
    #[msg("Proposal voting period has ended")]
    ProposalVotingEnded,
    
    // I require a waiting period before executing proposals
    #[msg("Proposal execution delay not satisfied")]
    ExecutionDelayNotSatisfied,
    
    // I need a minimum participation for governance decisions
    #[msg("Quorum threshold not reached")]
    QuorumNotReached,
    
    // I won't use oracle data that's too old
    #[msg("Oracle update exceeds maximum age")]
    StaleOracleData,
    
    // I need my backup oracle to work properly
    #[msg("Secondary oracle validation failed")]
    SecondaryOracleFailure,
    
    // I need to calculate fees accurately
    #[msg("Fee calculation error")]
    FeeCalculationError,
}