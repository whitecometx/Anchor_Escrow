use anchor_lang::prelude::*;

/// Defines the data stored for an escrow, which includes:
/// - a seed,
/// - maker's public key,
/// - token types (`mint_a` and `mint_b`),
/// - the expected receive amount,
/// - and a bump seed for address generation security.
#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub recieve: u64, // amount
    pub bump: u8
}