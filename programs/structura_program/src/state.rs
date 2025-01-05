use anchor_lang::prelude::*;

#[account]
pub struct ProjectAccount {
    pub name: String,
    pub budget: u64,
    pub milestones_completed: u8,
}

impl ProjectAccount {
    pub const LEN: usize = 8  // Discriminator
        + 32                  // name (up to 32 bytes in this example)
        + 8                   // budget (u64)
        + 1;                  // milestones_completed (u8)
}
