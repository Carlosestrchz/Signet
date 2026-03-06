# Signet 

**Proof of Provenance Protocol on Solana**

Signet es un protocolo descentralizado construido en la red de Solana que actúa como un notario criptográfico estricto. Permite a los usuarios registrar la huella digital única (HASH SHA-256) de sus archivos estáticos para demostrar su autoría y existencia en una línea de tiempo inmutable.

## El Problema que Resuelve
En la era de la IA generativa y la manipulación de medios, demostrar el origen y la autenticidad de un archivo digital es un desafío crítico. Signet resuelve el problema de confianza eliminando a los intermediarios centralizados y utilizando la inmutabilidad de la blockchain.

**Enfoque de Notario Estricto:** Signet no almacena los archivos para evitar altos costos y problemas de compresión, asi como tampoco previene el riesgo de *"bit rot"* de los archivos procesados (lo cual lleva a que el HASH del archivo original cambie) sino que se limita a certificar matemáticamente.
Evitar el "bit rot" es responsabilidad del usuario (aunque la plataforma permite revocar el HASH cuando ya no es valido).

## Arquitectura y Tecnologías
- **Blockchain:** Solana (Alta velocidad, bajas comisiones).
- **Smart Contract:** Rust.
- **Framework:** Anchor.
- **Estructura de Datos:** Program Derived Addresses (PDAs) para garantizar unicidad y evitar colisiones/plagio en el registro de archivos.

## Funcionalidades (MVP)
1. **Registrar Contenido (Create):** Genera una cuenta única (PDA) basada en el HASH del archivo. Si alguien intenta registrar exactamente el mismo documento, la red rechaza la transacción matemáticamente.
2. **Validar Autenticidad (Read):** Permite consultar la blockchain para confirmar el autor (Public Key) y el Timestamp (sello de tiempo) exacto en que el archivo fue validado.
3. **Revocar Validez (Update):** El autor original puede marcar un archivo como `revoked` (ej. por corrupción del archivo original o incumplimiento de contrato) y adjuntar una razón inmutable, sin destruir el historial criptográfico.
4. **Eliminar Registro (Delete):** El autor original tiene el poder absoluto de cerrar la cuenta (PDA) del archivo, eliminando el registro del estado actual de la blockchain y recuperando los fondos de almacenamiento (Rent).

## Estructura del Proyecto
El contrato inteligente sigue un patrón de diseño modular estricto:
- `src/lib.rs`: Enrutador de instrucciones (Endpoints).
- `src/state.rs`: Definición de las cuentas y asignación de memoria.
- `src/instructions.rs`: Lógica de negocio matemática y validación de seguridad.