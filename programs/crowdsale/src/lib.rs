use anchor_lang::prelude::*;

mod state;
mod constants;
mod instructions;

declare_id!("HciPz9qoNEBBWga6KWomnDovANbQWnTAT5iFSNW7Ji3K");

#[program]
pub mod crowdsale {
    pub use super::instructions::*;
    use super::*;

    // our Constructor
    pub fn initialize(ctx: Context<CreateCrowdsale>, id: Pubkey, const: u32) -> Result<()> {
      instructions::create_crowdsale(ctx, id, cost)
    }
        
        // Where a user will buy a token


        // Where a user or owner can withdraw SOL

}

#[derive(Accounts)]
pub struct Initialize {}
