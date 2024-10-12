use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::Bank;

#[derive(Accounts)]
pub struct InitBank<'Info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        space = 8 + Bank::INIT_SPACE
        seeds = [mint.key().as_ref()],
        bump
    )]
    pub bank: Account<'info, Bank>,

    #[account(
        init,
        token::mint = mint,
        token:authority = bank_token_account,
        payer = signer,
        seeds = [
            b"treasury",
            mint.key().as_ref()
        ],
        bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn process_init_bank(ctx: Context<InitBank>) -> Result<()> {
    let bank = &mut ctx.accounts.bank;
}
