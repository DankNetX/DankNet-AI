#[program]
pub mod dank_governance {
    use super::*;

    pub fn sanctify_meme(ctx: Context<SanctifyMeme>) -> Result<()> {
        let proposal = &mut ctx.accounts.meme_proposal;
        
        // Verify meme meets dankness criteria
        require!(
            proposal.chaos_score >= 6.9, 
            GovernanceError::NotDankEnough
        );
        
        // Check community votes
        let total_votes = proposal.yes_votes.checked_sub(proposal.no_votes)
            .ok_or(GovernanceError::VoteUnderflow)?;
        
        if total_votes >= SANCTIFICATION_THRESHOLD {
            // Mint Soulbound NFT
            let cpi_accounts = MintTo {
                mint: ctx.accounts.sanctified_mint.to_account_info(),
                to: ctx.accounts.voter.to_account_info(),
                authority: ctx.accounts.governance_authority.to_account_info(),
            };
            
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                cpi_accounts
            );
            
            token::mint_to(cpi_ctx, 1)?;
            
            // Add to eternal meme registry
            proposal.status = MemeStatus::Sanctified;
        }
        
        Ok(())
    }
}
