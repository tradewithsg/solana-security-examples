use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Good44444444444444444444444444444444444444");

#[program]
pub mod unsafe_cpi_secure {
    use super::*;

    /// ✅ SECURE INSTRUCTION
    ///
    /// Transfers tokens via CPI and verifies the result.
    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        let before_balance = ctx.accounts.vault_token.amount;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_token.to_account_info(),
                to: ctx.accounts.user_token.to_account_info(),
                authority: ctx.accounts.vault_authority.to_account_info(),
            },
        );

        token::transfer(cpi_ctx, amount)?;

        let after_balance = ctx.accounts.vault_token.amount;

        // ✅ Verify CPI effect
        require!(
            before_balance
                .checked_sub(amount)
                == Some(after_balance),
            ProgramError::InvalidCpiResult
        );

        vault.balance = vault
            .balance
            .checked_sub(amount)
            .ok_or(ProgramError::InvalidCpiResult)?;

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

#[error_code]
pub enum ProgramError {
    #[msg("Invalid CPI result")]
    InvalidCpiResult,
}
