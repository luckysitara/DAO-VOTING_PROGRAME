use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub votes: u32,
}
