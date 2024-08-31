//Event PDA to store event details on which the event is placed
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event { 
    pub event_id: Pubkey,
    pub event_name: String,
    pub event_result: PredictionOption,
}