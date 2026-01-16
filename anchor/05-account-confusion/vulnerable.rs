use anchor_lang::prelude::*;

declare_id!("Bad55555555555555555555555555555555555555");

#[program]
pub mod account_confusion {
    use super::*;

    /// ❌ VULNERABLE INSTRUCTION
    ///
    /// Updates user data using an untyped account.
    /// The bug: it accepts AccountInfo without validation.
    pub fn update_data(
        ctx: Context<UpdateData>,
        new_value: u64,
    ) -> Result<()> {
        let account_info = &ctx.accounts.user_data;

        // ❌ Blindly write to raw account data
        let mut data = account_info.try_borrow_mut_data()?;
        data[0..8].copy_from_slice(&new_value.to_le_bytes());

        Ok(())
    }
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    /// ❌ AccountInfo provides no guarantees about:
    /// - ownership
    /// - data layout
    /// - initialization
    pub user_data: AccountInfo<'info>,
}
