use anchor_lang::prelude::*;

declare_id!("2VcdhRBET2stSvf8G6Cq92N7q9LvApTkr9BL5hV4v1d1");

#[program]
pub mod dimm_smart_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
