# Stablecoin Protocol

## Overview
A decentralized stablecoin protocol built on Solana, providing a robust mechanism for collateralized token minting with advanced risk management, multi-oracle price feeds, governance capabilities, and sophisticated liquidation strategies.

## System Architecture
```mermaid
graph TD
    A[User] --> B[Stablecoin Protocol]
    B --> C[Config Management]
    B --> D[Collateral Tracking]
    B --> E[Multi-Oracle Integration]
    B --> F[Advanced Liquidation]
    B --> G[Governance System]
    B --> H[Fee Management]
    
    C --> C1[System Parameters]
    D --> D1[Collateral Accounts]
    E --> E1[Primary Pyth Oracle]
    E --> E2[Backup Oracle]
    F --> F1[Health Factor Calculation]
    F --> F2[Protocol Fee Collection]
    G --> G1[Proposal Creation]
    G --> G2[Voting Mechanism]
    G --> G3[Proposal Execution]
    H --> H1[Mint Fees]
    H --> H2[Burn Fees]
    H --> H3[Liquidation Fees]
```

## Key Features

### Core Features
- Collateralized Stablecoin Minting
- Dynamic Health Factor Calculation
- Advanced Liquidation Mechanism
- Multi-Oracle Price Feed Integration
- Secure Solana Program Architecture

### Enhanced Features
- **Multi-Oracle Support**: Redundant price feeds with deviation checks
- **Advanced Risk Management**: Volatility adjustments and collateralization limits
- **Decentralized Governance**: Community-driven parameter adjustments
- **Fee Structure**: Revenue generation through mint, burn, and liquidation fees
- **Improved Security**: Enhanced validation and error handling

## Detailed Mechanisms

### 1. Multi-Oracle Price Validation

```mermaid
flowchart TD
    A[Price Request] --> B{Primary Oracle Available?}
    B -->|Yes| C[Fetch Primary Price]
    B -->|No| D[Fail with Error]
    C --> E{Backup Oracle Available?}
    E -->|Yes| F[Fetch Backup Price]
    E -->|No| G[Use Primary Price Only]
    F --> H{Price Deviation Check}
    H -->|Deviation > Threshold| I[Fail with Error]
    H -->|Deviation <= Threshold| J[Calculate Average Price]
    G --> K[Apply Confidence Check]
    J --> K
    K -->|Confidence >= Threshold| L[Return Validated Price]
    K -->|Confidence < Threshold| M[Fail with Error]
```

#### Key Improvements
- **Redundancy**: Multiple price oracles for increased reliability
- **Deviation Checks**: Ensures price consistency across sources
- **Confidence Validation**: Requires high confidence in price data
- **Fallback Mechanism**: Graceful handling when one oracle is unavailable

### 2. Enhanced Health Factor Calculation

```mermaid
flowchart TD
    A[Start: Collateral Position] --> B[Fetch Validated Price]
    B --> C[Calculate Collateral Value in USD]
    C --> D[Apply Volatility Adjustment]
    D --> E[Apply Liquidation Threshold]
    E --> F[Calculate Minted Token Amount]
    F --> G[Calculate Health Factor]
    G --> H{Health Factor Evaluation}
    H -->|HF >= Min Threshold| I[Position Safe]
    H -->|Min Threshold > HF >= Critical| J[Warning: At Risk]
    H -->|HF < Critical| K[Liquidation Triggered]
    J --> L[Continue with Warning]
    I --> M[Continue Normal Operations]
    K --> N[Initiate Liquidation Process]
```

#### Mathematical Formulation
```
Adjusted Collateral Value = Collateral Value * (1 - Volatility Adjustment)
Health Factor = (Adjusted Collateral Value * Liquidation Threshold) / Minted Amount
```

### 3. Advanced Liquidation Mechanism

```mermaid
flowchart TD
    A[Detect Unhealthy Position] --> B[Validate Health Factor]
    B --> C[Calculate Liquidation Amount]
    C --> D[Calculate Liquidation Bonus]
    D --> E[Calculate Protocol Fee]
    E --> F[Compute Final Liquidation Value]
    F --> G[Transfer Collateral to Liquidator]
    G --> H[Transfer Fee to Protocol Treasury]
    H --> I[Burn Stablecoins]
    I --> J[Update Collateral Account]
    J --> K[Verify Final Health Factor]
```

#### Mathematical Formulation
```
Liquidation Base Amount = Stablecoin Amount * Current Price
Liquidation Bonus = Base Amount * Bonus Percentage
Protocol Fee = Base Amount * Fee Percentage
Final Liquidation Value = Base Amount + Bonus - Fee
```

### 4. Governance System

```mermaid
flowchart TD
    A[Community Member] --> B[Create Proposal]
    B --> C{Sufficient Token Balance?}
    C -->|No| D[Reject Proposal]
    C -->|Yes| E[Activate Proposal]
    E --> F[Voting Period]
    F --> G{Voting Complete}
    G --> H{Quorum Reached?}
    H -->|No| I[Proposal Fails]
    H -->|Yes| J{More Yes than No?}
    J -->|No| K[Proposal Rejected]
    J -->|Yes| L[Proposal Passed]
    L --> M[Execution Delay]
    M --> N[Execute Proposal]
    N --> O[Update Protocol Parameters]
```

#### Governance Parameters
- **Proposal Threshold**: 100,000 tokens
- **Voting Period**: 24 hours
- **Execution Delay**: 12 hours
- **Quorum Requirement**: 10% of total supply

### 5. Fee Structure

```mermaid
flowchart TD
    A[User Operation] --> B{Operation Type}
    B -->|Mint| C[Calculate Mint Fee: 0.1%]
    B -->|Burn| D[Calculate Burn Fee: 0.05%]
    B -->|Liquidate| E[Calculate Liquidation Fee: 0.5%]
    C --> F[Deduct Fee from Mint Amount]
    D --> G[Deduct Fee from Burn Amount]
    E --> H[Deduct Fee from Liquidation Amount]
    F --> I[Transfer Fee to Treasury]
    G --> I
    H --> I
    I --> J[Protocol Revenue]
```

## Technical Components

### Enhanced Rust Module Structure
- `state.rs`: Core data structures
- `constants.rs`: System configuration constants
- `error.rs`: Comprehensive error handling
- `instructions/`: Core program logic modules
  - `deposit/`: Collateral deposit and token minting
  - `withdraw/`: Collateral redemption and token burning
  - `admin/`: Administrative functions
  - `governance/`: Decentralized governance system
- `utils.rs`: Enhanced utility functions with multi-oracle support

### Key Data Structures

```mermaid
classDiagram
    class Config {
        +authority: Pubkey
        +mint_account: Pubkey
        +liquidation_threshold: u64
        +liquidation_bonus: u64
        +min_health_factor: u64
        +bump: u8
        +bump_mint_account: u8
    }
    
    class Collateral {
        +depositer: Pubkey
        +sol_account: Pubkey
        +token_account: Pubkey
        +lamport_balance: u64
        +amount_minted: u64
        +bump: u8
        +bump_sol_account: u8
        +is_initialized: bool
    }
    
    class Proposal {
        +id: u64
        +proposer: Pubkey
        +proposal_type: ProposalType
        +description: String
        +votes_for: u64
        +votes_against: u64
        +created_at: i64
        +voting_ends_at: i64
        +status: ProposalStatus
        +bump: u8
    }
    
    class ProposalType {
        <<enumeration>>
        UpdateMinHealthFactor(u64)
        UpdateLiquidationThreshold(u64)
        UpdateLiquidationBonus(u64)
        UpdateOracleConfig(u64)
        UpdateFeeStructure
    }
    
    class ProposalStatus {
        <<enumeration>>
        Active
        Passed
        Executed
        Rejected
        Cancelled
    }
    
    Config "1" -- "*" Proposal : governs
    Collateral "*" -- "1" Config : references
```

## Security Enhancements

- **Multi-Oracle Validation**: Prevents oracle manipulation attacks
- **Volatility Adjustment**: Accounts for market volatility in collateral valuation
- **Collateralization Limits**: Prevents both under and over-collateralization
- **Enhanced Error Handling**: Comprehensive error types with descriptive messages
- **Fee Collection**: Sustainable protocol revenue generation
- **Governance Timelock**: Prevents rushed or malicious parameter changes

## Protocol Parameters

| Parameter | Value | Description |
|-----------|-------|-------------|
| Liquidation Threshold | 50% | Maximum percentage of collateral that can be borrowed against |
| Liquidation Bonus | 10% | Incentive for liquidators |
| Min Health Factor | 1.0 | Minimum required health factor before liquidation |
| Critical Health Factor | 2.0 | Warning threshold for at-risk positions |
| Min Collateral Ratio | 150% | Minimum required collateralization |
| Max Collateral Ratio | 300% | Maximum recommended collateralization |
| Volatility Adjustment | 5% | Safety discount applied to collateral value |
| Mint Fee | 0.1% | Fee charged on minting operations |
| Burn Fee | 0.05% | Fee charged on burning operations |
| Liquidation Fee | 0.5% | Fee charged on liquidations |

## License
MIT License