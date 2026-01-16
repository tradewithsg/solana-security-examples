use anchor_lang::prelude::*;

declare_id!("Good11111111111111111111111111111111111111");

const MAX_NAME_LENGTH: usize = 50;

#[program]
pub mod missing_account_validation_secure {
    use super::*;

    /// ✅ SECURE VERSION
    ///
    /// Updates a user profile name with proper ownership validation.
    pub fn update_profile(
        ctx: Context<UpdateProfile>,
        new_name: String,
    ) -> Result<()> {
        require!(
            new_name.len() <= MAX_NAME_LENGTH,
            ProgramError::InvalidNameLength
        );
        require!(
            !new_name.trim().is_empty(),
            ProgramError::EmptyName
        );

        let profile = &mut ctx.accounts.profile;
        profile.name = new_name;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    /// ✅ Ownership and authority enforced
    #[account(
        mut,
        has_one = owner
    )]
    pub profile: Account<'info, UserProfile>,

    /// ✅ Authorized signer
    pub owner: Signer<'info>,
}

#[account]
pub struct UserProfile {
    pub owner: Pubkey,
    pub name: String,
}

#[error_code]
pub enum ProgramError {
    #[msg("Name cannot exceed 50 characters")]
    InvalidNameLength,
    #[msg("Name cannot be empty")]
    EmptyName,
}
