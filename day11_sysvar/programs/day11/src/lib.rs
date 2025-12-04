use anchor_lang::prelude::*;
use anchor_lang::solana_program::{sysvar };
use anchor_lang::{ solana_program::sysvar::instructions};

declare_id!("BaaByxG745EDy4o7DdpyCvsq2CZSnQNX4fmvUkdEkkZf");

#[program]
pub mod day11 {
    use core::time;

    use anchor_lang::prelude::sysvar::recent_blockhashes::{self, RecentBlockhashes};
    use chrono::Datelike;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        msg!("Block timestamp {:?}", clock.unix_timestamp);
        Ok(())
    }

    pub fn get_day_of_the_week(ctx: Context<Initialize>) -> Result<()> {
        let clock = Clock::get()?;
        let time = clock.unix_timestamp;
        let date_time = chrono::DateTime::from_timestamp(time, 0).unwrap();
        msg!("Day of the week {:?}", date_time.weekday());
        Ok(())
    }

    pub fn get_blockhash(ctx: Context<VarAccount>) -> Result<()> {

        let accountInfo = &ctx.accounts.instruction_sysvar;
        let recent_blockhashes = RecentBlockhashes::from_account_info((accountInfo));
        let data = recent_blockhashes.unwrap();
        let data = data.last().unwrap();

        msg!("The recent block hash is: {:?}", data.blockhash);
        let rent = Rent::get()?;
        msg!("Rent: {:?}", rent);
        Ok(())
    }

    pub fn get_sys_vars(ctx:Context<SysVarAccount>)->Result<()>{
        let data = RecentBlockhashes::from_account_info(&ctx.accounts.block_hash_var).unwrap();
        let entry = data.last().unwrap();
        msg!("Block hash: {:?}", entry.blockhash);
        let rent = Rent::get()?;
        msg!("Rent: {:?}", rent);
        let epoch_schedule = EpochSchedule::get().expect("should get epoche schedule ");
        msg!("Epoch schedule: {:?}", epoch_schedule);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize  {

}

#[derive(Accounts)]
pub struct VarAccount <'info> {
    /// CHECK: readonly
    pub instruction_sysvar: AccountInfo<'info>,
}



#[derive(Accounts)]
pub struct SysVarAccount <'info> {
    /// CHECK: readonly
    pub block_hash_var: AccountInfo<'info>,
    /// CHECK:
    pub stake_history_var: AccountInfo<'info>,
}

