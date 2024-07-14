use anchor_lang::prelude::*;

declare_id!("Your_Program_ID_Here");

#[program]
mod voting {
    use super::*;
    use crate::instructions::*;
    use crate::state::*;

    #[state]
    pub struct Voting {
        pub proposals: Vec<Proposal>,
        pub voters: Vec<AccountId>,
        pub voting_open: bool,
        pub points: HashMap<AccountId, u32>,
    }

    impl Voting {
        pub fn initialize(&mut self, ctx: Context<Initialize>) -> ProgramResult {
            self.proposals = vec![];
            self.voters = vec![];
            self.voting_open = false;
            self.points = HashMap::new();
            Ok(())
        }

        pub fn create_proposal(&mut self, ctx: Context<CreateProposal>, title: String, description: String) -> ProgramResult {
            let proposal = Proposal {
                id: self.proposals.len() as u32,
                title,
                description,
                votes: 0,
            };
            self.proposals.push(proposal);
            Ok(())
        }

        pub fn cast_vote(&mut self, ctx: Context<CastVote>, proposal_id: u32) -> ProgramResult {
            if !self.voting_open {
                return Err(ProgramError::InvalidInstructionData);
            }
            if proposal_id as usize >= self.proposals.len() {
                return Err(ProgramError::InvalidArgument);
            }
            if self.voters.contains(&ctx.accounts.voter.key()) {
                return Err(ProgramError::InvalidArgument);
            }

            self.voters.push(ctx.accounts.voter.key());
            self.proposals[proposal_id as usize].votes += 1;

            // Reward points to voter (simplified example)
            let voter_id = ctx.accounts.voter.key();
            let points_entry = self.points.entry(voter_id).or_insert(0);
            *points_entry += 1;

            Ok(())
        }

        pub fn close_voting(&mut self, ctx: Context<CloseVoting>) -> ProgramResult {
            self.voting_open = false;
            Ok(())
        }
    }
}
