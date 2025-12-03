use anchor_lang::prelude::*;

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
