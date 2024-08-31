//PDA to store bet details of a bettor

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub bettor: Pubkey,
    pub option: PredictionOption,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum PredictionOption {
    Yes,
    No,
}

impl From<u8> for PredictionOption {
    fn from(val: u8) -> Self {
        match val {
            0 => PredictionOption::No,
            1 => PredictionOption::Yes,
            _ => panic!("Invalid CurrencyType"),
        }
    }
}