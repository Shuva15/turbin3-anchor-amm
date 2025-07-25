#![allow(unexpected_cfgs, deprecated)]
pub mod constants;
pub mod error;
pub mod handlers;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use handlers::*;
pub use state::*;

declare_id!("7HHeuBaqfPt1KXoYgZ7ik7s9UcPdR7JeDk2rp4Ez1vSR");

#[program]
pub mod turbin3_anchor_amm {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, seed: u64, authority: Option<Pubkey>, fee: u16) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, asking_lp_amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(asking_lp_amount, max_x, max_y)
    }

    pub fn swap(ctx: Context<Swap>, is_deposit_token_x: bool, deposit_amount: u64) -> Result<()> {
        ctx.accounts.swap(is_deposit_token_x, deposit_amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, lp_token_amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(lp_token_amount, min_x, min_y)
    }

    pub fn update_locked(ctx: Context<Update>, want_to_lock: bool) -> Result<()> {
        if want_to_lock {
            ctx.accounts.lock()
        } else {
            ctx.accounts.unlock()
        }
    }
}
