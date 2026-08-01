use anchor_lang::prelude::*;
use crate::state::ProgramState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = ProgramState::LEN,
        seeds = [b"program_state"],
        bump
    )]
    pub program_state: Account<'info, ProgramState>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let program_state = &mut ctx.accounts.program_state;

    program_state.authority = ctx.accounts.authority.key();
    program_state.total_commitments = 0;
    program_state.version = 1;
    program_state.is_paused = false;
    program_state.reserved = [0; 64];

    msg!("EEMP Voting Program initialized");
    msg!("Authority: {}", program_state.authority);

    Ok(())
}
