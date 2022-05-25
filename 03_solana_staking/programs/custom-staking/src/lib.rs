use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::convert::TryInto;

#[cfg(not(feature = "local-testing"))]
declare_id!("DVmQmFfj8aD2KgJpkTt8q9cPB4ESYo8Qf8fJqHMUt7yr");

#[cfg(feature = "local-testing")]
declare_id!("DVmQmFfj8aD2KgJpkTt8q9cPB4ESYo8Qf8fJqHMUt7yr");

#[cfg(not(feature = "local-testing"))]
pub mod constants {
    pub const STEP_TOKEN_MINT_PUBKEY: &str = "teST1ieLrLdr4MJPZ7i8mgSCLQ7rTrPRjNnyFdHFaz9";
}

#[cfg(feature = "local-testing")]
pub mod constants {
    pub const STEP_TOKEN_MINT_PUBKEY: &str = "teST1ieLrLdr4MJPZ7i8mgSCLQ7rTrPRjNnyFdHFaz9";
}

const STATUS_STAKING: u8 = 1;
const STATUS_UNSTAKING: u8 = 2;
const STAKING_PDA_SEED: &[u8] = b"staking";

#[program]
pub mod custom_staking {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        lock_time: u64,
    ) -> Result<()> {
        ctx.accounts.staking_account.initializer_key = *ctx.accounts.initializer.key;
        ctx.accounts.staking_account.lock_time = lock_time;

        Ok(())
    }

    pub fn update_lock_time(
        ctx: Context<UpdateStakingAccountField>,
        new_lock_time: u64,
    ) -> Result<()> {
        ctx.accounts.staking_account.lock_time = new_lock_time;

        Ok(())
    }

    pub fn toggle_freeze_program(ctx: Context<FreezeProgram>) -> Result<()> {
        ctx.accounts.staking_account.freeze_program = !ctx.accounts.staking_account.freeze_program;

        Ok(())
    }

    pub fn stake(
        ctx: Context<Stake>,
        amount: u64,
    ) -> Result<()> {
        if ctx.accounts.user_staking_account.status == STATUS_UNSTAKING {
            return Err(ErrorCode::InvalidRequest.into());
        }
        let total_token = ctx.accounts.token_vault.amount;
        let total_x_token = ctx.accounts.staking_account.total_x_token;
        //mint x tokens
        if total_token == 0 || total_x_token == 0 {
            ctx.accounts.staking_account.total_x_token =
                (ctx.accounts.staking_account.total_x_token as u128)
                    .checked_add(amount as u128)
                    .unwrap()
                    .try_into()
                    .unwrap();
            ctx.accounts.user_staking_account.x_token_amount =
                (ctx.accounts.user_staking_account.x_token_amount as u128)
                    .checked_add(amount as u128)
                    .unwrap()
                    .try_into()
                    .unwrap();
        } else {
            let what: u64 = (amount as u128)
                .checked_mul(total_x_token as u128)
                .unwrap()
                .checked_div(total_token as u128)
                .unwrap()
                .try_into()
                .unwrap();

            ctx.accounts.staking_account.total_x_token =
                (ctx.accounts.staking_account.total_x_token as u128)
                    .checked_add(what as u128)
                    .unwrap()
                    .try_into()
                    .unwrap();
            ctx.accounts.user_staking_account.x_token_amount =
                (ctx.accounts.user_staking_account.x_token_amount as u128)
                    .checked_add(what as u128)
                    .unwrap()
                    .try_into()
                    .unwrap();
        }

        //transfer the users tokens to the vault
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.token_from.to_account_info(),
                to: ctx.accounts.token_vault.to_account_info(),
                authority: ctx.accounts.token_from_authority.to_account_info(),
            },
        );
        token::transfer(cpi_ctx, amount)?;

        (&mut ctx.accounts.token_vault).reload()?;

        //plus user staking amount
        ctx.accounts.user_staking_account.amount = (ctx.accounts.user_staking_account.amount
            as u128)
            .checked_add(amount as u128)
            .unwrap()
            .try_into()
            .unwrap();

        ctx.accounts.user_staking_account.status = STATUS_STAKING;

        Ok(())
    }

    pub fn restake(
        ctx: Context<Restake>,
    ) -> Result<()> {
        if ctx.accounts.user_staking_account.status != STATUS_UNSTAKING {
            return Err(ErrorCode::InvalidRequest.into());
        }

        let amount: u64 = ctx.accounts.user_staking_account.reward_amount;
        let total_token = ctx.accounts.token_vault.amount;
        let total_x_token = ctx.accounts.staking_account.total_x_token;
        //mint x tokens
        if total_token == 0 || total_x_token == 0 {
            ctx.accounts.staking_account.total_x_token =
                (ctx.accounts.staking_account.total_x_token as u128)
                    .checked_add(amount as u128)
                    .unwrap()
                    .try_into()
                    .unwrap();
            ctx.accounts.user_staking_account.x_token_amount =
                (ctx.accounts.user_staking_account.x_token_amount as u128)
                    .checked_add(amount as u128)
                    .unwrap()
                    .try_into()
                    .unwrap();
        } else {
            let what: u64 = (amount as u128)
                .checked_mul(total_x_token as u128)
                .unwrap()
                .checked_div(total_token as u128)
                .unwrap()
                .try_into()
                .unwrap();

            ctx.accounts.staking_account.total_x_token =
                (ctx.accounts.staking_account.total_x_token as u128)
                    .checked_add(what as u128)
                    .unwrap()
                    .try_into()
                    .unwrap();
            ctx.accounts.user_staking_account.x_token_amount =
                (ctx.accounts.user_staking_account.x_token_amount as u128)
                    .checked_add(what as u128)
                    .unwrap()
                    .try_into()
                    .unwrap();
        }

        //plus user staking amount
        ctx.accounts.user_staking_account.amount = (ctx.accounts.user_staking_account.amount
            as u128)
            .checked_add(amount as u128)
            .unwrap()
            .try_into()
            .unwrap();

        ctx.accounts.user_staking_account.status = STATUS_STAKING;
        ctx.accounts.user_staking_account.reward_amount = 0;
        ctx.accounts.user_staking_account.unstake_time = 0;

        Ok(())
    }

    pub fn unstake(
        ctx: Context<Unstake>,
    ) -> Result<()> {
        if ctx.accounts.user_staking_account.status != STATUS_STAKING {
            return Err(ErrorCode::InvalidRequest.into());
        }
        let now_ts = Clock::get().unwrap().unix_timestamp;
        let amount = ctx.accounts.user_staking_account.x_token_amount;

        let total_token = ctx.accounts.token_vault.amount;
        let total_x_token = ctx.accounts.staking_account.total_x_token;

        //burn what is being sent
        ctx.accounts.staking_account.total_x_token = (ctx.accounts.staking_account.total_x_token
            as u128)
            .checked_sub(amount as u128)
            .unwrap()
            .try_into()
            .unwrap();

        let what:u64 = (amount as u128)
            .checked_mul(total_token as u128)
            .unwrap()
            .checked_div(total_x_token as u128)
            .unwrap()
            .try_into()
            .unwrap();

        ctx.accounts.user_staking_account.amount = 0;
        ctx.accounts.user_staking_account.x_token_amount = 0;
        ctx.accounts.user_staking_account.reward_amount = what;
        ctx.accounts.user_staking_account.status = STATUS_UNSTAKING;
        ctx.accounts.user_staking_account.unstake_time = now_ts as u64;

        Ok(())
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        nonce_vault: u8,
    ) -> Result<()> {
        let now_ts: u64 = Clock::get().unwrap().unix_timestamp as u64;
        let lock_time = ctx.accounts.staking_account.lock_time;

        if ctx.accounts.user_staking_account.status != STATUS_UNSTAKING ||
            (now_ts < lock_time + ctx.accounts.user_staking_account.unstake_time)
        {
            return Err(ErrorCode::InvalidRequest.into());
        }

        let reward_amount = ctx.accounts.user_staking_account.reward_amount;

        //compute vault signer seeds
        let token_mint_key = ctx.accounts.token_mint.key();
        let seeds = &[token_mint_key.as_ref(), &[nonce_vault]];
        let signer = &[&seeds[..]];

        //transfer from vault to user
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.token_vault.to_account_info(),
                to: ctx.accounts.token_to.to_account_info(),
                authority: ctx.accounts.token_vault.to_account_info(),
            },
            signer,
        );
        token::transfer(cpi_ctx, reward_amount)?;

        (&mut ctx.accounts.token_vault).reload()?;

        ctx.accounts.user_staking_account.amount = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
    address = constants::STEP_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
    init,
    payer = initializer,
    token::mint = token_mint,
    token::authority = token_vault, //the PDA address is both the vault account and the authority (and event the mint authority)
    seeds = [ constants::STEP_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap().as_ref() ],
    bump,
    )]
    ///the not-yet-created, derived token vault pubkey
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    init,
    payer = initializer,
    seeds = [ STAKING_PDA_SEED.as_ref() ],
    bump,
    space = 8 + StakingAccount::LEN
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(mut)]
    ///pays rent on the initializing accounts
    pub initializer: Signer<'info>,

    ///used by anchor for init of the token
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateStakingAccountField<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,

    #[account(
    mut,
    seeds = [ STAKING_PDA_SEED.as_ref() ],
    bump,
    constraint = staking_account.initializer_key == initializer.key(),
    )]
    pub staking_account: Account<'info, StakingAccount>,
}

#[derive(Accounts)]
pub struct FreezeProgram<'info> {
    pub initializer: Signer<'info>,

    #[account(
    mut,
    seeds = [ STAKING_PDA_SEED.as_ref() ],
    bump,
    constraint = staking_account.initializer_key == *initializer.key,
    )]
    pub staking_account: Account<'info, StakingAccount>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
    address = constants::STEP_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    //the token account to withdraw from
    pub token_from: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    //the authority allowed to transfer from token_from
    pub token_from_authority: Signer<'info>,

    #[account(
    mut,
    seeds = [ token_mint.key().as_ref() ],
    bump,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    seeds = [ STAKING_PDA_SEED.as_ref() ],
    bump,
    constraint = !staking_account.freeze_program,
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(
    init_if_needed,
    payer = token_from_authority,
    seeds = [ token_from_authority.key().as_ref()],
    bump,
    space = 8 + UserStakingAccount::LEN
    )]
    pub user_staking_account: Account<'info, UserStakingAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Restake<'info> {
    #[account(
    address = constants::STEP_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    //the authority allowed to transfer from x_token_from
    pub x_token_from_authority: Signer<'info>,

    #[account(
    mut,
    seeds = [ token_mint.key().as_ref() ],
    bump,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    seeds = [ STAKING_PDA_SEED.as_ref() ],
    bump,
    constraint = !staking_account.freeze_program,
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(
    mut,
    seeds = [ x_token_from_authority.key().as_ref()],
    bump,
    )]
    pub user_staking_account: Account<'info, UserStakingAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(
    address = constants::STEP_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    //the authority allowed to transfer from x_token_from
    pub x_token_from_authority: Signer<'info>,

    #[account(
    mut,
    seeds = [ token_mint.key().as_ref() ],
    bump,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    seeds = [ STAKING_PDA_SEED.as_ref() ],
    bump,
    constraint = !staking_account.freeze_program,
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(
    mut,
    seeds = [ x_token_from_authority.key().as_ref()],
    bump,
    )]
    pub user_staking_account: Account<'info, UserStakingAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(nonce_vault: u8)]
pub struct Withdraw<'info> {
    #[account(
    address = constants::STEP_TOKEN_MINT_PUBKEY.parse::<Pubkey>().unwrap(),
    )]
    pub token_mint: Box<Account<'info, Mint>>,

    //the authority allowed to transfer from x_token_from
    pub x_token_from_authority: Signer<'info>,

    #[account(
    mut,
    seeds = [ token_mint.key().as_ref() ],
    bump,
    )]
    pub token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
    mut,
    seeds = [ STAKING_PDA_SEED.as_ref() ],
    bump,
    constraint = !staking_account.freeze_program,
    )]
    pub staking_account: Account<'info, StakingAccount>,

    #[account(
    mut,
    seeds = [ x_token_from_authority.key().as_ref()],
    bump,
    close=x_token_from_authority
    )]
    pub user_staking_account: Account<'info, UserStakingAccount>,

    #[account(mut)]
    //the token account to send token
    pub token_to: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(Default)]
pub struct StakingAccount {
    pub initializer_key: Pubkey,
    pub lock_time: u64,
    pub total_x_token: u64,
    pub freeze_program: bool,
}

impl StakingAccount {
    const LEN: usize = 32 + 8 + 8 + 1;
}

#[account]
#[derive(Default)]
pub struct UserStakingAccount {
    pub amount: u64,
    pub x_token_amount: u64,
    pub reward_amount: u64,
    pub unstake_time: u64,
    pub status: u8,
}

impl UserStakingAccount {
    const LEN: usize = 8 + 8 + 8 + 8 + 1;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid request")]
    InvalidRequest,
}
