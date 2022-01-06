use anchor_lang::prelude::*;

declare_id!("57p2dLPetHZKhBw78PbMPwiW1fLwfEMziA8xLd3r5kvm");

#[program]
pub mod todo_list {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
