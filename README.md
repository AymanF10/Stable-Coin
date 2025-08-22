# Stablecoin

## Overview
A decentralized stablecoin protocol built on Solana, providing a robust mechanism for collateralized token minting with advanced risk management and liquidation strategies.

## System Architecture
```mermaid
graph TD
    A[User] --> B[Stablecoin Protocol]
    B --> C[Config Management]
    B --> D[Collateral Tracking]
    B --> E[Price Oracle Integration]
    B --> F[Liquidation Mechanism]

    C --> G[System Parameters]
    D --> H[Collateral Accounts]
    E --> I[Pyth Price Feed]
    F --> J[Health Factor Calculation]
```

## Key Features
- Collateralized Stablecoin Minting
- Dynamic Health Factor Calculation
- Advanced Liquidation Mechanism
- Pyth Network Price Oracle Integration
- Secure Solana Program Architecture

## Detailed Calculations

### 1. Health Factor Calculation

#### Flowchart of Health Factor Determination
```mermaid
flowchart TD
    A[Start: Collateral Position] --> B[Fetch Current Collateral Value]
    B --> C[Retrieve Current Market Price]
    C --> D[Calculate Total Collateral Value in USD]
    D --> E[Apply Liquidation Threshold]
    E --> F[Calculate Minted Token Amount]
    F --> G{Health Factor Calculation}
    G --> |Health Factor Calculation| H[Collateral Value * Liquidation Threshold / Minted Amount]
    H --> I{Health Factor Comparison}
    I --> |Health Factor >= 1| J[Position Safe]
    I --> |Health Factor < 1| K[Liquidation Triggered]
    J --> L[Continue Normal Operations]
    K --> M[Initiate Liquidation Process]
```

#### Purpose
The health factor is a critical risk metric that determines the stability of a user's collateralized position. It ensures that the value of collateral adequately covers the minted stablecoins.

#### Mathematical Formulation
```
Health Factor = (Collateral Value * Liquidation Threshold) / Minted Amount
```

#### Detailed Breakdown
- **Collateral Value**:
  - Calculated by multiplying the collateral amount by its current market price
  - Obtained through real-time price feeds from Pyth Network
  - Represents the total USD value of locked collateral

- **Liquidation Threshold**:
  - A predefined percentage (typically 50-80%)
  - Represents the maximum percentage of collateral value that can be used for minting
  - Provides a safety buffer to protect against market volatility

- **Minted Amount**:
  - Total number of stablecoins issued against the collateral
  - Directly impacts the health of the position

### 2. Liquidation Calculation

#### Liquidation Process Flowchart
```mermaid
flowchart TD
    A[Detect Unhealthy Position] --> B[Calculate Minted Token Value]
    B --> C[Determine Base Liquidation Amount]
    C --> D[Calculate Liquidation Bonus]
    D --> E[Compute Total Liquidation Value]
    E --> F[Initiate Collateral Seizure]
    F --> G[Transfer Collateral to Liquidator]
    G --> H[Burn Minted Tokens]
    H --> I[Update Collateral Account]
    I --> J[End Liquidation Process]
```

#### Purpose
Liquidation mechanism protects the protocol from under-collateralized positions by allowing liquidators to seize and sell collateral.

#### Mathematical Formulation
```
Liquidation Base Amount = Minted Token Amount * Current Price
Liquidation Bonus = Liquidation Base Amount * Bonus Percentage
Total Liquidation Value = Base Amount + Liquidation Bonus
```

#### Detailed Breakdown
- **Minted Token Amount**:
  - Converted to USD using current market price
  - Determines the base liquidation value

- **Liquidation Bonus**:
  - Incentive percentage for liquidators
  - Typically 5-10% of the liquidation value
  - Encourages prompt identification and resolution of risky positions

### 3. Price Conversion Calculations

#### USD to Lamports Conversion Flowchart
```mermaid
flowchart TD
    A[Input USD Amount] --> B[Fetch Current SOL Price]
    B --> C[Multiply by LAMPORTS_PER_SOL]
    C --> D[Divide by Current SOL Price]
    D --> E[Output Lamports Amount]
```

#### Lamports to USD Conversion Flowchart
```mermaid
flowchart TD
    A[Input Lamports Amount] --> B[Fetch Current SOL Price]
    B --> C[Multiply by Current SOL Price]
    C --> D[Divide by LAMPORTS_PER_SOL]
    D --> E[Output USD Amount]
```

#### Conversion Formulas
```
Lamports = (USD Amount * LAMPORTS_PER_SOL) / Current SOL Price
USD Value = (Lamports * Current SOL Price) / LAMPORTS_PER_SOL
```

#### Key Considerations
- Uses Pyth Network's real-time price feeds
- Accounts for decimal precision
- Handles potential price volatility

## Technical Components

### Rust Module Structure
- `config.rs`: System configuration management
- `collateral.rs`: User collateral tracking
- `utils.rs`: Utility functions for price conversion
- `instructions/`: Core program logic modules

### Key Data Structures
```mermaid
classDiagram
    class ConfigAccount {
        +authority: Pubkey
        +mint_account: Pubkey
        +liquidation_threshold: u64
        +liquidation_bonus: u64
        +min_health_factor: u64
    }
    
    class CollateralAccount {
        +depositer: Pubkey
        +lamport_balance: u64
        +amount_minted: u64
        +is_initialized: bool
    }
```

## Security Considerations
- Dynamic health factor calculation
- Robust liquidation mechanism
- Pyth oracle price validation
- Strict collateralization requirements

## License
MIT License




