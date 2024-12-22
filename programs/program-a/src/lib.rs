use anchor_lang::prelude::*;
use program_b::program::ProgramB;

declare_id!("87e1rrxPrWHYih5KhxkdaN5Mb3SHZk6w1JbVP9sgyhv5");

#[program]
pub mod program_a {
    use anchor_lang::solana_program::{program::invoke_signed, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from program A");
        let pda_address = ctx.accounts.pda_account.key();
        let signer_address = ctx.accounts.signer.key();
        let bump = ctx.bumps.pda_account;

        let instruction =
            &system_instruction::transfer(&pda_address, &signer_address, 1_000_000_000);
        let account_infos = [
            ctx.accounts.pda_account.to_account_info(),
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ];

        let signer_seeds: &[&[&[u8]]] = &[&[b"vector", signer_address.as_ref(), &[bump]]];

        invoke_signed(instruction, &account_infos, signer_seeds)?;

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.program_b.to_account_info(),
            program_b::cpi::accounts::Initialize {
                pda_account: ctx.accounts.pda_account.to_account_info(),
            },
            signer_seeds,
        );

        program_b::cpi::initialize(cpi_context)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vector", signer.key().as_ref()],
        bump,

    )]
    /// CHECK: This account does not require any type checks for now
    pub pda_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub program_b: Program<'info, ProgramB>,
}
