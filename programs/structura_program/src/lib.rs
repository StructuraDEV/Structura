use anchor_lang::prelude::*;

pub mod instruction_handlers;
pub mod state;
pub mod utils;

use instruction_handlers::*;

declare_id!("STuraYourProgramIDHere00000000000000000000000000");

#[program]
pub mod structura_program {
    use super::*;

    /// Initializes a project with a name, total budget, and no milestones completed yet.
    pub fn initialize_project(
        ctx: Context<InitializeProject>,
        name: String,
        budget: u64,
    ) -> Result<()> {
        initialize_project_handler(ctx, name, budget)
    }

    /// Updates a project milestone (mark complete, or add partial completion details).
    pub fn update_milestone(
        ctx: Context<UpdateMilestone>,
        milestone_index: u8,
        completed: bool,
    ) -> Result<()> {
        update_milestone_handler(ctx, milestone_index, completed)
    }

    /// (Placeholder) Distributes rewards to a project’s participants from the project’s treasury or a token mint.
    pub fn distribute_rewards(
        ctx: Context<DistributeRewards>,
        amount: u64,
    ) -> Result<()> {
        distribute_rewards_handler(ctx, amount)
    }
}
