use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use super::StakingInstance;
use super::User;
use anchor_spl::token::Mint;
use super::Metadata;

#[derive(Accounts)]
#[instruction(
    token_per_sec: u64,
    staking_instance_bump: u8,
)]
pub struct CancelStaking<'info> {
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    #[account(
        mut,
        constraint = nft_token_metadata.collection.unwrap().key == 
            allowed_collection_address.key(),
    )]
    pub nft_token_metadata: Box<Account<'info, Metadata>>,
    #[account(mut)]
    pub reward_token_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub reward_token_authority_wallet: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = nft_token_metadata.mint == nft_token_mint.key(),
    )]
    pub nft_token_mint: Box<Account<'info, Mint>>,
    #[account(mut)]
    pub nft_token_authority_wallet: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub nft_token_program_wallet: Box<Account<'info, TokenAccount>>,
    #[account(
        mut, 
        seeds = [b"stakingInstance".as_ref()],
        bump = staking_instance_bump,
        constraint = allowed_collection_address.key() 
            == staking_instance.allowed_collection_address,
    )]
    pub staking_instance: Account<'info, StakingInstance>,
    #[account(
        init, 
        seeds = [b"stakingUser".as_ref()],
        bump = staking_instance_bump,
        space = 8 + core::mem::size_of::<User>(),
        payer = authority,
    )]
    pub user_instance: Account<'info, User>,
    pub allowed_collection_address: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub rent: AccountInfo<'info>,
    pub time: Sysvar<'info,Clock>,
    pub token_program: AccountInfo<'info>,

}

