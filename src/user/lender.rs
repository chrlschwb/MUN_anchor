use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::token::{
    self, spl_token::instruction::AuthorityType, CloseAccount, Mint, SetAuthority, Token,
    TokenAccount, Transfer,
};

// use solana_program::program::invoke;
// use solana_program::native_token::LAMPORTS_PER_SOL;
const AUTHORITY_SEED: &[u8] = b"authority";

pub fn transfer_data_save(
    ctx: Context<TransferSolWithCpi>,
    payer: Pubkey,
    amount: u64,
    interest: u8,
    offer_percent_floor: u8,
    end_date: i64,
    nft_collection_id: Vec<String>,
) -> Result<()> {
    let clock = Clock::get()?;
    msg!("Current Timestamp: {}", clock.unix_timestamp);
    ctx.accounts.data.payer = payer;
    ctx.accounts.data.amount = amount;
    ctx.accounts.data.interest = interest;
    ctx.accounts.data.offer_percent_floor = offer_percent_floor;
    ctx.accounts.data.end_date = end_date;
    ctx.accounts.data.nft_collection_id = nft_collection_id;
    ctx.accounts.data.create_date = clock.unix_timestamp;

    Ok(())
}

pub fn transfer_sol_with_cpi(
    ctx: Context<TransferSolWithCpi>,
    amount: u64,
    interest: u8,
    offer_percent_floor: u8,
    _create_date: i64,
    end_date: i64,
    nft_collection_id: Vec<String>,
) -> Result<()> {
    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.recipient.to_account_info(),
            },
        ),
        amount,
    )?;

    let payer = ctx.accounts.payer.key();
    transfer_data_save(
        ctx,
        payer,
        amount,
        interest,
        offer_percent_floor,
        end_date,
        nft_collection_id,
    )
}

pub fn withdraw_sol(ctx: Context<WithdrawSol>, amount: u64) -> Result<()> {
    let ix = anchor_lang::solana_program::system_instruction::transfer(
        &ctx.accounts.from.key(),
        &ctx.accounts.to.key(),
        amount,
    );
    anchor_lang::solana_program::program::invoke(
        &ix,
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.to.to_account_info(),
        ],
    );

    Ok(())
}

#[derive(Accounts)]
pub struct TransferSolWithCpi<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    #[account(mut)]
    recipient: SystemAccount<'info>,

    system_program: Program<'info, System>,

    #[account(mut)]
    pub data: Account<'info, AcountData>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut, signer)]
    /// CHECK: This is not dangerous because we just pay to this account
    from: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we just pay to this account
    to: AccountInfo<'info>,
    #[account()]
    pub user: Signer<'info>,
    system_program: AccountInfo<'info>,
}

#[account]
pub struct AcountData {
    payer: Pubkey,
    amount: u64,
    interest: u8,
    offer_percent_floor: u8,
    create_date: i64,
    end_date: i64,
    nft_collection_id: Vec<String>,
}
