use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub event: Account<'info, Event>,
    #[account(
        init,
        payer = signer,
        seeds = [b"bet", event.event_id().as_ref(), signer.key().as_ref()],
        bump,
        space = 8 + Bet::INIT_SPACE,
    )]
    pub bet: Account<'info, Bet>,
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, amount:u64) -> Result<()> {
        Ok(())
    }
}