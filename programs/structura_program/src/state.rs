use anchor_lang::prelude::*;

#[account]
pub struct ProjectAccount {
    pub name: String,
    pub budget: u64,
    pub milestones_completed: u8,
}

impl ProjectAccount {
    pub const LEN: usize = 8 // Discriminator
        + 32                // name
        + 8                 // budget
        + 1;                // milestones_completed
}
