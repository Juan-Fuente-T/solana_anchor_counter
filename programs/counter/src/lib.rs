use anchor_lang::prelude::*;

declare_id!("8aYT4mngcGqR3KqHEzbU2RLhqHqkime8xVibsBYqVSEG");

#[program]
mod counter {
    use super::*;

    pub fn crear_contador(ctx: Context<Crear>, primer_numero: u64) -> Result<()> {
        ctx.accounts.contador.numero = primer_numero;
        ctx.accounts.contador.autoridad = ctx.accounts.autoridad.key();
        msg!("creando un contador con numero inicial {} ", primer_numero);
        Ok(())
    }

    pub fn aumentar_contador(ctx: Context<Incrementar>, numero: u64) -> Result<()>{
        ctx.accounts.contador.numero += numero;
        msg!("Incrementando el numero del contador en esta cantidad {} ", numero);
        Ok(())
    }
    pub fn incrementar_contador(ctx: Context<Incrementar>) -> Result<()>{
        ctx.accounts.contador.numero += 1;
        msg!("Incrementando el numero del contador en 1 ");
        Ok(())
    }
    pub fn borrar_contador(_ctx: Context<Borrar>) -> Result<()>{
         msg!("Contador eliminado");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Crear<'info> {
    // 8 bytes para discriminador  + (lo que ocupe tu estructura)
    #[account(init, payer = autoridad, space = 8 + 8 + 32)]
    pub contador: Account<'info, Contador>,
    #[account(mut)]
    pub autoridad: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
    pub struct Incrementar<'info>{
    #[account(mut)]
    pub autoridad: Signer<'info>,
    #[account(
        mut,
        constraint = contador.autoridad == contador.key()
    )]
    pub contador: Account<'info, Contador>,
}

#[derive(Accounts)]
pub struct Borrar<'info> {
    #[account(mut)]
    pub autoridad: Signer<'info>,
    #[account(
        mut,
        constraint = contador.autoridad == contador.key(),
        close = autoridad
    )]
    pub contador: Account<'info, Contador>,
}

#[account]
pub struct Contador {
    numero: u64, // 8 bytes
    autoridad: Pubkey, //32 butes
}

#[error_code]
pub enum ErrorCode{
    #[msg("You are not auyhorized.")]
    NotAuthorized,
}

