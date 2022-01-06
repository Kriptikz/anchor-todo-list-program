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
pub struct InitializeList {}

#[account]
pub struct TodoList {

}
