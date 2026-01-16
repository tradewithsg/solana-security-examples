use anchor_lang::prelude::*;

declare_id!("Good33333333333333333333333333333333333333");

#[program]
pub mod unsafe_arithmetic_secure {
    use super::*;

    /// ✅ SECURE INSTRUCTION
    ///
    /// Withdraws funds from a wallet using checked arithmetic.
    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        wallet.balance = wallet
            .balance
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub wallet: Account<'info, Wallet>,
}

#[account]
pub struct Wallet {
    pub balance: u64,
}

#[error_code]
pub enum ProgramError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
