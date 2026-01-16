use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Bad44444444444444444444444444444444444444");

#[program]
pub mod unsafe_cpi {
    use super::*;

    /// ❌ VULNERABLE INSTRUCTION
    ///
    /// Transfers tokens via CPI and updates internal balance.
    /// The bug: it assumes the CPI behaved correctly.
    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        // ❌ Blindly trust CPI
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
        );

        token::transfer(cpi_ctx, amount)?;

        // ❌ Assume tokens were actually transferred
        vault.balance -= amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    #[account(mut)]
    pub vault_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token: Account<'info, TokenAccount>,

    /// CHECK: simplified for example
    pub vault_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub balance: u64,
}
