use anchor_lang::prelude::*;

declare_id!("DyzRkkVCTyjm3bSuLHF6yVWd3f9TGrAJnvDFhDBTEmk5");

#[program]
pub mod prema {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
