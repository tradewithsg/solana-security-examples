use anchor_lang::prelude::*;

declare_id!("Good22222222222222222222222222222222222222");

#[program]
pub mod missing_authority_check_secure {
    use super::*;

    /// ✅ SECURE INSTRUCTION
    ///
    /// Withdraws funds from a vault with proper authority validation.
    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
    ) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        vault.balance = vault
            .balance
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    /// ✅ Ownership enforced
    #[account(
        mut,
        has_one = owner
    )]
    pub vault: Account<'info, Vault>,

    /// ✅ Authorized signer
    pub owner: Signer<'info>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
}

#[error_code]
pub enum ProgramError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
}
