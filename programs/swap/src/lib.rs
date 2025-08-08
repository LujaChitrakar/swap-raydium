use anchor_lang::prelude::*;
use anchor_lang::solana_program::{instruction::Instruction, program::invoke_signed};
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};

declare_id!("BUNi68phLfzQEQLLxHSPoodcgUMQJzsEshENP2dDKy4a");

const RAYDIUM_AMM_PROGRAM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";

#[program]
pub mod swap {
    use super::*;

    pub fn initialize(ctx: Context<InitializeProgram>) -> Result<()> {
        Ok(())
    }

    pub fn swap(ctx: Context<SwapTokens>, amount_in: u64, min_amount_out: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_source_token_account.to_account_info(),
                to: ctx.accounts.program_source.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        transfer(cpi_ctx, amount_in)?;

        let mut data: Vec<u8> = vec![];
        data.push(9u8);
        data.extend_from_slice(&amount_in.to_le_bytes());
        data.extend_from_slice(&min_amount_out.to_le_bytes());

        let swap_instruction = Instruction {
            program_id: ctx.accounts.raydium_program.key(),
            accounts: vec![
                AccountMeta::new(ctx.accounts.amm_id.key(), false),
                AccountMeta::new_readonly(ctx.accounts.amm_authority.key(), false),
                AccountMeta::new(ctx.accounts.amm_open_orders.key(), false),
                AccountMeta::new(ctx.accounts.amm_target_orders.key(), false),
                AccountMeta::new(ctx.accounts.pool_coin_token_account.key(), false),
                AccountMeta::new(ctx.accounts.pool_pc_token_account.key(), false),
                AccountMeta::new_readonly(ctx.accounts.serum_program.key(), false),
                AccountMeta::new(ctx.accounts.serum_market.key(), false),
                AccountMeta::new(ctx.accounts.serum_bids.key(), false),
                AccountMeta::new(ctx.accounts.serum_asks.key(), false),
                AccountMeta::new(ctx.accounts.serum_event_queue.key(), false),
                AccountMeta::new(ctx.accounts.serum_coin_vault_account.key(), false),
                AccountMeta::new(ctx.accounts.serum_pc_vault_account.key(), false),
                AccountMeta::new_readonly(ctx.accounts.serum_vault_signer.key(), false),
                AccountMeta::new(ctx.accounts.program_source.key(), false),
                AccountMeta::new(ctx.accounts.program_destination.key(), false),
                AccountMeta::new_readonly(ctx.accounts.user.key(), true),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
            ],
            data: data,
        };

        let signer_seeds: &[&[&[u8]]] = &[&[b"authority", &[ctx.bumps.program_authority]]];

        invoke_signed(
            &swap_instruction,
            &[
                ctx.accounts.amm_id.to_account_info(),
                ctx.accounts.amm_authority.to_account_info(),
                ctx.accounts.amm_open_orders.to_account_info(),
                ctx.accounts.amm_target_orders.to_account_info(),
                ctx.accounts.pool_coin_token_account.to_account_info(),
                ctx.accounts.pool_pc_token_account.to_account_info(),
                ctx.accounts.serum_program.to_account_info(),
                ctx.accounts.serum_market.to_account_info(),
                ctx.accounts.serum_bids.to_account_info(),
                ctx.accounts.serum_asks.to_account_info(),
                ctx.accounts.serum_event_queue.to_account_info(),
                ctx.accounts.serum_coin_vault_account.to_account_info(),
                ctx.accounts.serum_pc_vault_account.to_account_info(),
                ctx.accounts.serum_vault_signer.to_account_info(),
                ctx.accounts.program_source.to_account_info(),
                ctx.accounts.program_destination.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
            ],
            signer_seeds,
        )?;

        let output_amount = ctx.accounts.program_destination.amount;

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.program_destination.to_account_info(),
                to: ctx
                    .accounts
                    .user_destination_token_account
                    .to_account_info(),
                authority: ctx.accounts.program_authority.to_account_info(),
            },
            signer_seeds,
        );

        transfer(cpi_ctx, output_amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    ///CHECK:PDA authority
    #[account(
        init,
        payer=authority,
        space=8,
        seeds=[b"authority"],
        bump
    )]
    pub program_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SwapTokens<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        constraint=user_source_token_account.owner==user.key()
    )]
    pub user_source_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=user_destination_token_account.owner==user.key()
    )]
    pub user_destination_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA authority
    #[account(seeds=[b"authority"],bump)]
    pub program_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint=program_source.owner==program_authority.key(),
        constraint=program_source.mint==user_source_token_account.mint
    )]
    pub program_source: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint=program_destination.owner==program_authority.key(),
        constraint=program_destination.mint==user_destination_token_account.mint
    )]
    pub program_destination: Account<'info, TokenAccount>,

    // Raydium accounts
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub amm_id: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub amm_target_orders: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub pool_coin_token_account: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub pool_pc_token_account: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    pub serum_program: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub serum_market: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub serum_bids: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub serum_asks: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub serum_event_queue: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub serum_coin_vault_account: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    #[account(mut)]
    pub serum_pc_vault_account: UncheckedAccount<'info>,
    /// CHECK: Verified by raydium
    pub serum_vault_signer: UncheckedAccount<'info>,

    /// CHECK: Verified by raydium
    #[account(
        constraint=raydium_program.key().to_string()==RAYDIUM_AMM_PROGRAM
    )]
    pub raydium_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
