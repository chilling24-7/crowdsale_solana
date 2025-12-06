$[account]
pub struct Crowdsale{
    pub id: PubKey,
    
    pub cost: u32,

    pub mint_account: PubKey,

    pub token_account: Pubkey,

    pub status: CrowdsaleStatus,

    pub owner: PubKey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, ParialEq, Eq)]

pub enum Crowdsale {
    Open, Closed
}

impl Crowdsale {
    pub const MAXIMUM_SIZE: usize = 32 + 4 + 32 + 32 + 1 + 32;
}
