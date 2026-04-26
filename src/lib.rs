use anchor_lang::prelude::*;

// ID del programa (Se genera al hacer build en SolPG)
declare_id!("Hbo6qQjFn6uQcFJTvVWUkyWLzWEkPsiJ61X7E378xTFd");

#[program]
pub mod arcade_hall_solana {
    use super::*;

    // 1. CREATE (PDA): Inicializa el Salón de la Fama dándole nombre al local
    pub fn inicializar_arcade(ctx: Context<CrearArcade>, nombre_arcade: String) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger;
        ledger.owner = ctx.accounts.owner.key();
        ledger.nombre_arcade = nombre_arcade;
        ledger.records = Vec::new();
        
        msg!("Salón de la Fama para '{}' inicializado.", ledger.nombre_arcade);
        Ok(())
    }

    // 2. CREATE (Dato): Registra un récord exigiendo juego, puntos y jugador
    pub fn registrar_record(
        ctx: Context<GestionarArcade>, 
        juego: String, 
        puntuacion: u32, 
        jugador: String
    ) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger;
        require!(ledger.owner == ctx.accounts.owner.key(), Errores::NoAutorizado);

        let nuevo_record = Record {
            nombre_juego: juego,
            puntuacion_maxima: puntuacion,
            nombre_jugador: jugador,
        };

        ledger.records.push(nuevo_record);
        msg!("¡Nuevo récord registrado en la blockchain!");
        Ok(())
    }

    // 3. UPDATE: Modifica un récord existente (Requiere todos los campos actualizados)
    pub fn editar_record(
        ctx: Context<GestionarArcade>, 
        juego: String, 
        nueva_puntuacion: u32, 
        nuevo_jugador: String
    ) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger;
        require!(ledger.owner == ctx.accounts.owner.key(), Errores::NoAutorizado);

        let lista = &mut ledger.records;
        for i in 0..lista.len() {
            if lista[i].nombre_juego == juego {
                lista[i].puntuacion_maxima = nueva_puntuacion;
                lista[i].nombre_jugador = nuevo_jugador;
                msg!("Récord de '{}' actualizado exitosamente.", juego);
                return Ok(());
            }
        }
        Err(Errores::RecordNoEncontrado.into())
    }

    // 4. DELETE: Elimina un récord del Salón de la Fama
    pub fn eliminar_record(ctx: Context<GestionarArcade>, juego: String) -> Result<()> {
        let ledger = &mut ctx.accounts.ledger;
        require!(ledger.owner == ctx.accounts.owner.key(), Errores::NoAutorizado);

        let lista = &mut ledger.records;
        let index = lista.iter().position(|r| r.nombre_juego == juego);

        if let Some(i) = index {
            lista.remove(i);
            msg!("Récord de '{}' eliminado del sistema.", juego);
            Ok(())
        } else {
            Err(Errores::RecordNoEncontrado.into())
        }
    }

    // 5. READ: Visualiza todos los récords del arcade
    pub fn ver_records(ctx: Context<GestionarArcade>) -> Result<()> {
        msg!("Arcade: {}", ctx.accounts.ledger.nombre_arcade);
        msg!("Lista de Honor: {:#?}", ctx.accounts.ledger.records);
        Ok(())
    }
}

// --- ESTADO DEL PROGRAMA ---

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Record {
    #[max_len(25)]
    pub nombre_juego: String,
    pub puntuacion_maxima: u32,
    #[max_len(25)]
    pub nombre_jugador: String,
}

#[account]
#[derive(InitSpace)]
pub struct ArcadeLedger {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_arcade: String,
    #[max_len(15)] // Capacidad para 15 récords históricos
    pub records: Vec<Record>,
}

// --- CONTEXTOS ---

#[derive(Accounts)]
pub struct CrearArcade<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = 8 + ArcadeLedger::INIT_SPACE,
        seeds = [b"arcade", owner.key().as_ref()],
        bump
    )]
    pub ledger: Account<'info, ArcadeLedger>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarArcade<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub ledger: Account<'info, ArcadeLedger>,
}

// --- ERRORES ---

#[error_code]
pub enum Errores {
    #[msg("No tienes permisos para modificar este Salón de la Fama.")]
    NoAutorizado,
    #[msg("El récord solicitado no existe en el registro.")]
    RecordNoEncontrado,
}
