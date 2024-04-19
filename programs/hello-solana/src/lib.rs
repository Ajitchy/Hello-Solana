// use this import to gain access to common anchor features
use anchor_lang::prelude::*;

//declare an id for your program
declare_id!("CJ9nEHtNFvidMGjSgBazjd1WynrWK9Sg17NDt9bN2HpN");

//write your business logic here
#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

//validate incoming account here
#[derive(Accounts)]
pub struct Initialize {}
