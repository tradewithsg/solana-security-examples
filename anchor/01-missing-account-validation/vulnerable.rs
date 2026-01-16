use anchor_lang::prelude::*;

declare_id!("Bad111111111111111111111111111111111111111");

#[program]
pub mod missing_account_validation {
    use super::*;

    /// ❌ VULNERABLE INSTRUCTION
    ///
    /// This instruction updates a user's profile name.
    /// The bug is that it does NOT verify who owns the profile.
    ///
    /// Any user can pass ANY profile account and modify it.
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        new_name: String,
    ) -> Result<()> {
        let profile = &mut ctx.accounts.profile;

        // ❌ No ownership or authority validation
        profile.name = new_name;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    /// ❌ VULNERABILITY:
    /// This account has no constraints tying it to a specific user.
    /// Anchor will deserialize the account but will NOT check:
    /// - who owns it
    /// - who is allowed to modify it
    pub profile: Account<'info, UserProfile>,
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub name: String,
}
