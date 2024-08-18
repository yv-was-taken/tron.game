use anchor_lang::prelude::*;

declare_id!("4iyUz5fSLwMRRWHL5n7UNPU34thGC7vZHB4Ja74qf4tH");

#[program]
pub mod bolt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
