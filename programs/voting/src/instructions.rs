use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub proposal: Account<'info, Proposal>,
    pub user: Signer<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct CloseVoting<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: AccountInfo<'info>,
}
