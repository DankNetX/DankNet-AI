use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, TokenAccount};

#[derive(Accounts)]
pub struct RecursiveBurn<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority
    )]
    pub burn_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> RecursiveBurn<'info> {
    pub fn execute_burn(ctx: Context<Self>, meme_count: u64) -> Result<()> {
        let burn_rate = 0.0069; // 0.69% per meme
        let total_supply = ctx.accounts.mint.supply as f64;
        let burn_amount = (total_supply * burn_rate * meme_count as f64) as u64;

        // Create burn CPI context
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.burn_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        );

        // Execute initial burn
        token::burn(cpi_ctx, burn_amount)?;

        // Check for recursive burn conditions
        if ctx.accounts.mint.supply > 1_000_000_000 {
            let recursive_burn_amount = burn_amount / 2;
            token::burn(cpi_ctx.with_signer(&[]), recursive_burn_amount)?;
        }

        emit!(BurnEvent {
            burner: ctx.accounts.authority.key(),
            amount: burn_amount,
            remaining_supply: ctx.accounts.mint.supply
        });

        Ok(())
    }
}
