use anchor_lang::prelude::*;

declare_id!("57p2dLPetHZKhBw78PbMPwiW1fLwfEMziA8xLd3r5kvm");

#[program]
pub mod todo_list {

    use super::*;
    pub fn initialize_list(ctx: Context<InitializeList>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeList {
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Task {
    pub name: String,
    pub completed: bool,
    pub verified: bool,
    pub amount: u64,
}

#[account]
pub struct TodoList {
    pub authority: Pubkey,
    pub tasks: Vec<Task>,
    pub vault: Pubkey,
}
