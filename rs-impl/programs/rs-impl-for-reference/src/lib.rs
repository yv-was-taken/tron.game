use anchor_lang::prelude::*;

declare_id!("FX2JLp5jxde7Hnim7c4zANB7hWDkf7E7JEzuxWwK6GTR");

#[program]
pub mod rs_impl_for_reference {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
