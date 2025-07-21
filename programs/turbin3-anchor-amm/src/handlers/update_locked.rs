use anchor_lang::prelude::*;

use crate::{Config, error::AmmError};

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub authorized_user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config: Account<'info, Config>
}

impl<'info> Update<'info> {
    pub fn lock(&mut self) -> Result<()> {
        require!(
            self.authorized_user.key() == self.config.authority.unwrap(),
            AmmError::InvalidAuthority
        );
        self.config.locked = true;
        Ok(())
    }

    pub fn unlock(&mut self) -> Result<()> {
        require!(
            self.authorized_user.key() == self.config.authority.unwrap(),
            AmmError::InvalidAuthority
        );
        self.config.locked = false;
        Ok(())
    }
}