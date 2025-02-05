#[derive(Accounts)]
pub struct SanctifyMeme<'info> {
    #[account(mut)]
    pub proposal: Account<'info, MemeProposal>,
    #[account(mut)]
    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> SanctifyMeme<'info> {
    pub fn execute(ctx: Context<Self>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        
        // Dank threshold: 666 votes
        if proposal.votes >= 666 {
            proposal.status = ProposalStatus::Sanctified;
            
            // Mint Soulbound NFT
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.voter.to_account_info(),
                    authority: ctx.accounts.program_authority.to_account_info(),
                },
            );
            token::mint_to(cpi_ctx, 1)?;
        }
        Ok(())
    }
}
