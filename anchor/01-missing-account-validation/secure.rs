use anchor_lang::prelude::*;

declare_id!("Good11111111111111111111111111111111111111");

#[program]
pub mod missing_account_validation_secure {
    use super::*;

    /// ✅ SECURE INSTRUCTION
    ///
    /// This instruction updates a user's profile name,
    /// but now enforces proper ownership and authority checks.
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        new_name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        profile.name = new_name;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    /// ✅ FIX:
    /// - `has_one = owner` ensures the profile belongs to the signer
    /// - `mut` allows safe mutation
    #[account(
        mut,
        has_one = owner
    )]
    pub profile: Account<'info, UserProfile>,

    /// ✅ The signer who owns the profile
    pub owner: Signer<'info>,
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub name: String,
}
