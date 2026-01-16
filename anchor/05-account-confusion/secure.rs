use anchor_lang::prelude::*;

declare_id!("Good55555555555555555555555555555555555555");

#[program]
pub mod account_confusion_secure {
    use super::*;

    /// ✅ SECURE INSTRUCTION
    ///
    /// Updates user data using a strongly typed account.
    pub fn update_data(
        ctx: Context<UpdateData>,
        new_value: u64,
    ) -> Result<()> {
        let user_data = &mut ctx.accounts.user_data;

        user_data.value = new_value;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    /// ✅ Strongly typed account enforces:
    /// - correct owner
    /// - correct data layout
    /// - proper deserialization
    #[account(mut)]
    pub user_data: Account<'info, UserData>,
}

#[account]
pub struct UserData {
    pub value: u64,
}
