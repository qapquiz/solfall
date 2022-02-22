use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

declare_id!("BHeZ6pUpo8Z3d1x93mwn2SNnrddFSUYoPAdNdEfxX3d2");

#[program]
pub mod solfall {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deposit_initialize(ctx: Context<DepositInitialize>) -> Result<()> {
        let deposit_detail = &mut ctx.accounts.deposit_detail;
        deposit_detail.token_mint = ctx.accounts.token_mint.key();
        deposit_detail.deposit_amount = 0;
        deposit_detail.bump = *ctx.bumps.get("deposit_detail").unwrap();
        deposit_detail.owner = ctx.accounts.depositor.key();

        // @todo #3 create stake account for port financeg
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        // @todo #4 refresh reserve account
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct DepositInitialize<'info> {
    pub token_mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [
            b"deposit_detail".as_ref(),
            depositor.key().as_ref()
        ],
        bump,
        payer = depositor,
        space = DepositDetail::LEN,
    )]
    pub deposit_detail: Account<'info, DepositDetail>,
    #[account(mut)]
    pub depositor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {

}

// impl<'info> Deposit<'info> {
//     fn into_init_obligation_context<'a, 'b, 'c, 'info,
// }

#[account]
pub struct DepositDetail {
    pub token_mint: Pubkey,
    pub deposit_amount: u64,
    pub bump: u8,
    pub owner: Pubkey,
}

impl DepositDetail {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 32;
}