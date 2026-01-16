use anchor_lang::prelude::*;

declare_id!("Bad22222222222222222222222222222222222222");

#[program]
pub mod missing_authority_check {
    use super::*;

    /// ❌ VULNERABLE INSTRUCTION
    ///
    /// Withdraws funds from a vault.
    /// The bug: it does NOT verify that the caller is the vault owner.
    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        // ❌ No authority check
        vault.balance = vault.balance.saturating_sub(amount);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// ❌ The vault has an owner field,
    /// but the instruction never verifies who is calling.
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
}
