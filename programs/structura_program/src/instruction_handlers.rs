use anchor_lang::prelude::*;
use crate::state::*;
use crate::utils::*;

#[derive(Accounts)]
pub struct InitializeProject<'info> {
    #[account(init, payer = user, space = ProjectAccount::LEN)]
    pub project_account: Account<'info, ProjectAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_project_handler(ctx: Context<InitializeProject>, name: String, budget: u64) -> Result<()> {
    let project_account = &mut ctx.accounts.project_account;
    project_account.name = name;
    project_account.budget = budget;
    project_account.milestones_completed = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateMilestone<'info> {
    #[account(mut)]
    pub project_account: Account<'info, ProjectAccount>,
    pub user: Signer<'info>,
}

pub fn update_milestone_handler(ctx: Context<UpdateMilestone>, milestone_index: u8, completed: bool) -> Result<()> {
    let project_account = &mut ctx.accounts.project_account;
    if completed {
        project_account.milestones_completed += 1;
    }
    msg!("Milestone {} updated. Completed: {}", milestone_index, completed);
    Ok(())
}

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub project_account: Account<'info, ProjectAccount>,
    pub user: Signer<'info>,
}

pub fn distribute_rewards_handler(ctx: Context<DistributeRewards>, amount: u64) -> Result<()> {
    let project_account = &mut ctx.accounts.project_account;
    msg!("Distributing {} $STURA for project: {}", amount, project_account.name);
    Ok(())
}
