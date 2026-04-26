# 🕹️ Arcade Hall Solana


Sistema de gestión de récords arcade desarrollado como **Solana Program** utilizando **Rust** y el framework **Anchor**.  

Este proyecto implementa un sistema **CRUD** para registrar y administrar puntuaciones en un Salón de la Fama directamente en blockchain, garantizando:

- 🔑 Uso de Program Derived Addresses (PDAs)  
- ⚡ Optimización de memoria *On-Chain*  
- 🔒 Seguridad basada en firmas  

---

## 📚 Descripción

**Arcade Hall Solana** simula un sistema de puntuaciones donde un administrador puede:

- Inicializar un salón arcade  
- Registrar récords de distintos juegos  
- Actualizar puntuaciones y jugadores  
- Eliminar récords  
- Consultar la lista completa en blockchain  

---

## 🧠 Arquitectura y Estructuras de Datos

En Solana es necesario definir el tamaño de los datos para calcular correctamente la renta (*rent*).

### 📦 PDA Principal: `ArcadeLedger`

Cuenta raíz que almacena todos los récords del arcade.

```rust
#[account]
#[derive(InitSpace)]
pub struct ArcadeLedger {
    pub owner: Pubkey,
    #[max_len(40)]
    pub nombre_arcade: String,
    #[max_len(15)]
    pub records: Vec<Record>,
}
```

---

### 🧩 Estructura Interna: `Record`

Cada récord contiene:

- `nombre_juego (String)` → nombre del juego  
- `puntuacion_maxima (u32)` → mejor puntuación registrada  
- `nombre_jugador (String)` → jugador que logró el récord  

```rust
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Record {
    #[max_len(25)]
    pub nombre_juego: String,
    pub puntuacion_maxima: u32,
    #[max_len(25)]
    pub nombre_jugador: String,
}
```

---

## 🔒 Seguridad

El contrato valida que solo el propietario pueda modificar el sistema:

```rust
require!(
    ledger.owner == ctx.accounts.owner.key(),
    Errores::NoAutorizado
);
```

✔ Protege la integridad de los récords  
✔ Evita modificaciones no autorizadas  

---

## ⚙️ Funcionalidad (CRUD)

### 🟢 Inicializar Arcade

Crea la cuenta principal usando:

```rust
[b"arcade", owner.key().as_ref()]
```

Inicializa:
- Owner  
- Nombre del arcade  
- Lista vacía de récords  

---

### ➕ Registrar Récord

- Recibe:
  - juego  
  - puntuación  
  - jugador  
- Inserta en el vector con `.push()`  

---

### ✏️ Editar Récord

- Busca por `nombre_juego`  
- Actualiza:
  - puntuación  
  - jugador  

---

### ❌ Eliminar Récord

```rust
.iter().position(|r| r.nombre_juego == juego)
```

- Si existe → `.remove(index)`  
- Si no → error `RecordNoEncontrado`  

---

### 📖 Ver Récords

```rust
msg!("Lista de Honor: {:#?}", ledger.records);
```

Muestra todos los récords en logs *On-Chain*

---

## 🧪 Despliegue en Solana Playground

1. Copia el código en `lib.rs`  
2. Ejecuta:

```bash
cargo clean
```

3. Haz clic en **Build**  
4. Haz clic en **Deploy (Devnet)**  

---

## 🧑‍💻 Pruebas

Puedes interactuar con el contrato usando:

- Pestaña **Test** del Playground  
- Scripts en TypeScript:

```ts
pg.program.methods...
```

Parámetros:
- `juego: String`  
- `puntuacion: u32`  
- `jugador: String`  

---

## ⚠️ Manejo de Errores

```rust
#[error_code]
pub enum Errores {
    #[msg("No tienes permisos para modificar este Salón de la Fama.")]
    NoAutorizado,
    #[msg("El récord solicitado no existe en el registro.")]
    RecordNoEncontrado,
}
```

---

## 📌 Conclusión

Este proyecto demuestra:

- Gestión de rankings en blockchain  
- Seguridad mediante validación de firmas  
- Uso eficiente de estructuras dinámicas  
- Implementación de CRUD en un caso práctico (Arcade Hall)  

---

## 🚀 Próximos pasos

- Crear ranking automático por puntuación  
- Permitir múltiples récords por juego  
- Integrar frontend visual (leaderboard)  
- Añadir recompensas con tokens SPL  

---
