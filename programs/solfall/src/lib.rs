use anchor_lang::prelude::*;

declare_id!("BHeZ6pUpo8Z3d1x93mwn2SNnrddFSUYoPAdNdEfxX3d2");

#[program]
pub mod solfall {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(deposit_amount: u64)]
pub struct Deposit<'info> {
    // @todo #1 use #[account] init here with seeds = [b"deposit_detail", user_pubkey]
    pub deposit_detail: Account<'info, DepositDetail>,
}

#[account]
pub struct DepositDetail {
    pub token_mint: Pubkey,
    pub deposit_amount: u64,
    pub owner: Pubkey,
}
