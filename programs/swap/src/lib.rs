#![allow(deprecated)]
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::Metadata,
    token::{Mint, Token},
};

declare_id!("BUNi68phLfzQEQLLxHSPoodcgUMQJzsEshENP2dDKy4a");
declare_program!(raydium_launchpad);

use crate::raydium_launchpad::{program::RaydiumLaunchpad,cpi::{accounts::InitializeV2,initialize_v2}};

pub mod constants;
use constants::*;

#[program]
pub mod swap {

    use crate::raydium_launchpad::types::{AmmFeeOn, ConstantCurve, CurveParams, MintParams, VestingParams};

    use super::*;

    pub fn initialize<'info>(ctx:Context<'_,'_,'_,'info,Initialize<'info>>,token_args:TokenArgs)->Result<()>{
        let cpi_ctx=CpiContext::new(ctx.accounts.raydium_launchpad_program.to_account_info(), InitializeV2{
            payer:ctx.accounts.user.to_account_info(),
            creator:ctx.accounts.user.to_account_info(),
            global_config:ctx.accounts.global_config.to_account_info(),
            platform_config:ctx.accounts.platform_config.to_account_info(),
            authority:ctx.accounts.authority.to_account_info(),
            pool_state:ctx.accounts.pool_state.to_account_info(),
            base_mint:ctx.accounts.base_token_mint.to_account_info(),
            quote_mint:ctx.accounts.quote_token_mint.to_account_info(),
            base_vault:ctx.accounts.base_vault.to_account_info(),
            quote_vault:ctx.accounts.quote_vault.to_account_info(),
            metadata_account:ctx.accounts.metadata_account.to_account_info(),
            base_token_program:ctx.accounts.base_token_program.to_account_info(),
            quote_token_program:ctx.accounts.quote_token_program.to_account_info(),
            metadata_program:ctx.accounts.metadata_program.to_account_info(),
            system_program:ctx.accounts.system_program.to_account_info(),
            rent_program:ctx.accounts.rent_program.to_account_info(),
            event_authority:ctx.accounts.event_authority.to_account_info(),
            program:ctx.accounts.raydium_launchpad_program.to_account_info()
        });

        let base_mint_param=MintParams{
            name:token_args.name,
            symbol:token_args.symbol,
            uri:token_args.uri,
            decimals:6,

        };

        let curve_param=CurveParams::Constant { data: ConstantCurve {
            supply: 1000000000000000,
            total_base_sell: 793100000000000,
            total_quote_fund_raising: 12500000000,
            migrate_type: 1,
        },
     };

     let vesting_param=VestingParams{
        total_locked_amount:0,
        cliff_period:0,
        unlock_period:0
     };

     let amm_fee_on=AmmFeeOn::QuoteToken;

        initialize_v2(cpi_ctx, base_mint_param, curve_param, vesting_param, amm_fee_on)?;
        Ok(())
    }

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TokenArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user:Signer<'info>,

    ///CHECK: Raydium Launchpad Global Config
    #[account(
        mut,
        seeds=[GLOBAL_CONFIG_SEED,quote_token_mint.key().as_ref(),0u8.to_le_bytes().as_ref(),0u16.to_le_bytes().as_ref()],
        seeds::program=raydium_launchpad_program.key(), 
        bump,
    )]
    pub global_config:AccountInfo<'info>,

    ///CHECK: Platform Config from raydium launchpad program
    #[account(
        mut,
        seeds=[PLATFORM_CONFIG_SEED,user.key().as_ref()],
        seeds::program=raydium_launchpad_program.key(),
        bump,
    )]
    pub platform_config:AccountInfo<'info>,

    /// CHECK: raydium authority
    #[account(
        mut,
        seeds=[AUTH_SEED],
        seeds::program=raydium_launchpad_program.key(),
        bump
    )]
    pub authority:AccountInfo<'info>,

    ///CHECK: pool state checked by raydium program
    #[account(
        mut,
        seeds=[POOL_SEED,base_token_mint.key().as_ref(),quote_token_mint.key().as_ref()],
        seeds::program=raydium_launchpad_program.key(),
        bump
    )]
    pub pool_state:AccountInfo<'info>,

    pub quote_token_mint:Account<'info, Mint>,
    pub base_token_mint:Account<'info, Mint>,

    /// CHECK base vault checked by raydium
    #[account(
        mut,
        seeds=[POOL_VAULT_SEED,pool_state.key().as_ref(),base_token_mint.key().as_ref()],
        seeds::program=raydium_launchpad_program.key(),
        bump
    )]
    pub base_vault:AccountInfo<'info>,

    /// CHECK quote vault checked by raydium
    #[account(
        mut,
        seeds=[POOL_VAULT_SEED,pool_state.key().as_ref(),quote_token_mint.key().as_ref()],
        seeds::program=raydium_launchpad_program.key(),
        bump
    )]
    pub quote_vault:AccountInfo<'info>,

    #[account(
        mut,
        seeds=[METADATA_SEED,metadata_program.key().as_ref(),base_token_mint.key().as_ref()],
        seeds::program=metadata_program.key(),
        bump
    )]
    pub metadata_account:SystemAccount<'info>,
    
    ///CHECK raydium event authority
    #[account(
        mut,
        seeds=[EVENT_AUTHORITY],
        bump,
        seeds::program=raydium_launchpad_program.key()
    )]
    pub event_authority:AccountInfo<'info>,

    pub base_token_program:Program<'info,Token>,
    pub quote_token_program:Program<'info,Token>,

    pub metadata_program:Program<'info,Metadata>,
    pub system_program:Program<'info,System>,
    pub rent_program:Sysvar<'info,Rent>,
    pub raydium_launchpad_program:Program<'info,RaydiumLaunchpad>,
} 