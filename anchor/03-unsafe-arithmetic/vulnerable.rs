use anchor_lang::prelude::*;

declare_id!("Bad33333333333333333333333333333333333333");

#[program]
pub mod unsafe_arithmetic {
    use super::*;

    /// ❌ VULNERABLE INSTRUCTION
    /// Withdraws funds from a wallet.
    /// The bug: subtraction is performed without checking for underflow.
    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;

        // ❌ Unsafe arithmetic
        wallet.balance -= amount;

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
