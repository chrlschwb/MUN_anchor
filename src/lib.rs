use anchor_lang::prelude::*;

pub mod nft;
pub mod user;

use nft::*;
use user::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("5EtbjvXpjiazYcQwq9L3V5XMxyTBWtsdE7UqsYmtaG6f");

#[program]
mod mun_tool {
    use super::*;

    pub fn init(_ctx: Context<InitalAccount>) -> Result<()> {
        Ok(())
    }

    pub fn leader_create_data(
        ctx: Context<TransferSolWithCpi>,
        amount: u64,
        interest: u8,
        offer_percent_floor: u8,
        create_date: i64,
        end_date: i64,
        nft_collection_id: Vec<String>,
    ) -> Result<()> {
        user::lender::transfer_sol_with_cpi(
            ctx,
            amount,
            interest,
            offer_percent_floor,
            create_date,
            end_date,
            nft_collection_id,
        )
    }

    pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
        user::lender::withdraw_sol(ctx, amount)
    }

    pub fn create_mun_nft(
        ctx: Context<CreateToken>,
        nft_title: String,
        nft_symbol: String,
        nft_uri: String,
    ) -> Result<()> {
        nft::create::create_token(ctx, nft_title, nft_symbol, nft_uri)
    }

    pub fn mint_mun_nft(ctx: Context<MintTo>) -> Result<()> {
        nft::mint::mint_to(ctx)
    }

    pub fn transfer_tokens(ctx: Context<TransferTokens>, quantity: u64) -> Result<()> {
        nft::transfer::transfer_tokens(ctx, quantity)
    }

}

#[derive(Accounts)]
pub struct InitalAccount<'info> {
    #[account(init, space=8+8+64+32+8, payer=payer)]
    pub data: Account<'info, AcountData>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
