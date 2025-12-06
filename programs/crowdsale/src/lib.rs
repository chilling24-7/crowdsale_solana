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
        pub fn buy_tokens (ctx:  Context<BuyTokens, amount: u32) -> Result<()> {
          instructions::buy_tokens(ctx, amount)
        }

        // Where a user or owner can withdraw SOL
        pub fn withdraw(ctx: Context(Withdraw)) -> Result<()> {
          instructions::withdraw(ctx)
        }
}

#[derive(Accounts)]
pub struct Initialize {}
