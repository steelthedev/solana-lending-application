use anchor_lang::prelude::*;

pub struct Repay<'info> {
    #[account(mut)]
    pub signer: Signer<'info, Mint>,
}
