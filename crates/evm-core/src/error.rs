use thiserror::Error;

/// Errors produced by the `evm-core` crate.
#[derive(Debug, Error)]
pub enum EvmCoreError {
    /// The signature recovery id was not 0 or 1.
    #[error("invalid recovery id: expected 0 or 1, got {0}")]
    InvalidRecoveryId(u8),
}
