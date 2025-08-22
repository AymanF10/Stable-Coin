use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};

use crate::{
    Collateral, Config, CustomError, 
    FEED_ID, BACKUP_FEED_ID, MAXIMUM_AGE, PRICE_FEED_DECIMAL_ADJUSTMENT,
    ORACLE_CONFIDENCE_THRESHOLD, MAX_ORACLE_PRICE_DEVIATION, 
    CRITICAL_HEALTH_FACTOR, MAX_COLLATERAL_RATIO, MIN_COLLATERAL_RATIO,
    VOLATILITY_ADJUSTMENT, MAX_MINT_AMOUNT, BPS_DIVISOR
};

pub fn check_health_factor(
    collateral_amount: u64,
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<()> {
    let health_factor = calculate_health_factor(collateral_amount, collateral, config, price_feed)?;
    
    // I need to make sure the position is safe by checking against my minimum threshold
    require!(
        health_factor >= config.min_health_factor,
        CustomError::BelowMinHealthFactor
    );
    
    // I like to warn users when they're getting close to the danger zone
    if health_factor < CRITICAL_HEALTH_FACTOR {
        msg!("Warning: Health factor approaching critical level: {}", health_factor);
    }
    
    Ok(())
}

pub fn calculate_health_factor (
    collateral_amount: u64,
    collateral: &Account<Collateral>,
    config: &Account<Config>,
    price_feed: &Account<PriceUpdateV2>,
) -> Result<u64> {
    // I first get the USD value using my fancy multi-oracle setup for extra safety
    let collateral_value_in_usd = get_validated_usd_value(collateral_amount, price_feed)?;
    
    // I apply a volatility adjustment because crypto prices can swing wildly
    let volatility_adjusted_value = apply_volatility_adjustment(collateral_value_in_usd)?;
    
    // I use the liquidation threshold to determine how much of the collateral I'll count
    let collateral_adjusted_for_liquidation_threshold = 
        (volatility_adjusted_value * config.liquidation_threshold) / 100;

    // If they haven't minted anything yet, their position is perfectly safe
    if collateral.amount_minted == 0 {
        msg!("Health factor Max");
        return Ok(u64::MAX); 
    }
    
    // I check if they're over or under collateralized compared to my recommended ranges
    let collateralization_ratio = (collateral_value_in_usd * 100) / collateral.amount_minted;
    
    if collateralization_ratio > MAX_COLLATERAL_RATIO {
        msg!("Warning: Excessive collateralization: {}%", collateralization_ratio);
    } else if collateralization_ratio < MIN_COLLATERAL_RATIO {
        msg!("Warning: Low collateralization: {}%", collateralization_ratio);
    }
    
    // Finally, I calculate the health factor - this is the core of my risk management
    let health_factor = (collateral_adjusted_for_liquidation_threshold) / collateral.amount_minted;
    Ok(health_factor)
}

fn apply_volatility_adjustment(value: u64) -> Result<u64> {
    // I reduce the collateral value a bit to account for market volatility
    let adjusted_value = value * (100 - VOLATILITY_ADJUSTMENT) / 100;
    Ok(adjusted_value)
}

pub fn get_validated_usd_value(amount_in_lamports: u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    // I always start with my primary oracle - Pyth is pretty reliable
    let primary_price = get_price_from_feed(FEED_ID, price_feed)?;
    
    // I try to get a second opinion from my backup oracle
    let backup_price_result = get_price_from_feed(BACKUP_FEED_ID, price_feed);
    
    match backup_price_result {
        Ok(backup_price) => {
            // I check if my oracles are giving me consistent prices
            let deviation = calculate_price_deviation(primary_price, backup_price);
            
            // If they disagree too much, something fishy might be going on
            if deviation > MAX_ORACLE_PRICE_DEVIATION as u64 {
                msg!("Warning: Oracle price deviation: {}%", deviation);
                return Err(error!(CustomError::OraclePriceDeviation));
            }
            
            // Two heads are better than one - I use the average price for better accuracy
            let average_price = (primary_price + backup_price) / 2;
            let price_in_usd = average_price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;
            let amount_in_usd = (amount_in_lamports as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);
            Ok(amount_in_usd as u64)
        },
        Err(_) => {
            // If my backup oracle is down, I can still work with just the primary
            msg!("Warning: Using only primary oracle price feed");
            let price_in_usd = primary_price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;
            let amount_in_usd = (amount_in_lamports as u128 * price_in_usd) / (LAMPORTS_PER_SOL as u128);
            Ok(amount_in_usd as u64)
        }
    }
}

fn calculate_price_deviation(price1: u64, price2: u64) -> u64 {
    // I can't divide by zero, so I handle that case specially
    if price1 == 0 || price2 == 0 {
        return 100; // I consider this maximum deviation
    }
    
    // I find the higher and lower prices to calculate the percentage difference
    let max_price = std::cmp::max(price1, price2);
    let min_price = std::cmp::min(price1, price2);
    
    // I calculate how much the prices differ as a percentage
    ((max_price - min_price) * 100) / min_price
}

fn get_price_from_feed(feed_id_hex: &str, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    // I convert the hex feed ID to the format Pyth expects
    let feed_id = get_feed_id_from_hex(feed_id_hex)?;
    
    // I make sure the price isn't too old - stale prices are dangerous
    let price = price_feed.get_price_no_older_than(&Clock::get()?, MAXIMUM_AGE, &feed_id)?;
    
    // I need a positive price to work with
    require!(price.price > 0, CustomError::InvalidPrice);
    
    // I check how confident the oracle is in this price
    let confidence_percentage = (price.conf as u64 * 100) / price.price as u64;
    if confidence_percentage < ORACLE_CONFIDENCE_THRESHOLD as u64 {
        msg!("Warning: Low price confidence: {}%", confidence_percentage);
        return Err(error!(CustomError::LowOracleConfidence));
    }
    
    Ok(price.price as u64)
}

pub fn get_usd_value(amount_in_lamports: u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    // I use my enhanced validation for all USD conversions
    get_validated_usd_value(amount_in_lamports, price_feed)
}

pub fn get_lamports_from_usd(amount_in_usd: &u64, price_feed: &Account<PriceUpdateV2>) -> Result<u64> {
    // I don't allow minting beyond my safety limit
    require!(*amount_in_usd <= MAX_MINT_AMOUNT, CustomError::ExcessiveMintAmount);
    
    // I get the current price from my primary oracle
    let primary_price = get_price_from_feed(FEED_ID, price_feed)?;
    let price_in_usd = primary_price as u128 * PRICE_FEED_DECIMAL_ADJUSTMENT;
    
    // I convert the USD amount to lamports based on current price
    let amount_in_lamports = (*amount_in_usd as u128 * (LAMPORTS_PER_SOL as u128)) / price_in_usd;
    Ok(amount_in_lamports as u64)
}

pub fn calculate_fee(amount: u64, fee_bps: u16) -> Result<u64> {
    // I calculate fees in basis points (1/100th of a percent)
    let fee = (amount as u128 * fee_bps as u128) / BPS_DIVISOR as u128;
    
    // I make sure the fee doesn't overflow
    if fee > u64::MAX as u128 {
        return Err(error!(CustomError::FeeCalculationError));
    }
    
    Ok(fee as u64)
}