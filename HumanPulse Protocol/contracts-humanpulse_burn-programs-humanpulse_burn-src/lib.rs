use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Token, TokenAccount, Transfer};

declare_id!("BURN111111111111111111111111111111111111111");

#[program]
pub mod humanpulse_burn {

    use super::*;

    /// Un'istituzione paga `amount` HPP. Il contratto brucia il 50% e invia
    /// il restante 50% al validatore designato.
    pub fn process_verification_fee(ctx: Context<ProcessFee>, amount: u64) -> Result<()> {
        // --- 1. Trasferisci i token dal pagatore al contratto ---
        let transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer_token_account.to_account_info(),
                to: ctx.accounts.contract_token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        );
        token::transfer(transfer_ctx, amount)?;

        // --- 2. Calcola 50% (burn) e 50% (validatore) ---
        let burn_amount = amount / 2;
        let validator_amount = amount - burn_amount;

        // --- 3. Brucia il 50% ---
        let burn_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.mint.to_account_info(),
                from: ctx.accounts.contract_token_account.to_account_info(),
                authority: ctx.accounts.contract_signer.to_account_info(),
            },
        );
        token::burn(burn_ctx, burn_amount)?;

        // --- 4. Invia il restante 50% al validatore ---
        let validator_transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.contract_token_account.to_account_info(),
                to: ctx.accounts.validator_token_account.to_account_info(),
                authority: ctx.accounts.contract_signer.to_account_info(),
            },
        );
        token::transfer(validator_transfer_ctx, validator_amount)?;

        // --- 5. Emetti evento per tracciamento off‑chain ---
        emit!(VerificationProcessed {
            payer: ctx.accounts.payer.key(),
            validator: ctx.accounts.validator.key(),
            amount,
            burned: burn_amount,
            validator_reward: validator_amount,
        });

        Ok(())
    }
}

/// Evento emesso dopo ogni verifica
#[event]
pub struct VerificationProcessed {
    pub payer: Pubkey,
    pub validator: Pubkey,
    pub amount: u64,
    pub burned: u64,
    pub validator_reward: u64,
}

/// Account necessari per processare una fee di verifica
#[derive(Accounts)]
pub struct ProcessFee<'info> {
    /// Il pagatore (istituzione) – firma la transazione
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Token account del pagatore da cui prelevare gli HPP
    #[account(mut)]
    pub payer_token_account: Account<'info, TokenAccount>,

    /// Token account del contratto che riceve temporaneamente i fondi
    #[account(mut)]
    pub contract_token_account: Account<'info, TokenAccount>,

    /// Token account del validatore che riceverà il 50%
    #[account(mut)]
    pub validator_token_account: Account<'info, TokenAccount>,

    /// Il validatore (non firma, solo destinatario)
    /// CHECK: solo per indirizzo, non leggiamo dati
    #[account(mut)]
    pub validator: AccountInfo<'info>,

    /// Mint del token HPP
    #[account(mut)]
    pub mint: Account<'info, token::Mint>,

    /// Autorità del contratto (PDA che firma burn e transfer)
    /// CHECK: PDA generato dal programma
    #[account(seeds = [b"burner"], bump)]
    pub contract_signer: AccountInfo<'info>,

    /// Programma token SPL
    pub token_program: Program<'info, Token>,
}