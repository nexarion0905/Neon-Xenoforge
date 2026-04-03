#[derive(Debug)]
pub enum LedgerError {
    InsufficientFunds,
    TransactionFailed,
}

#[derive(Debug)]
pub enum ParadoxError {
    TemporalAnomaly,
    IdentityMismatch,
}

#[derive(Debug)]
pub enum IdentityError {
    IdentityNotFound,
    DuplicateIdentity,
}

#[derive(Debug)]
pub enum EntanglementError {
    UnresolvableState,
    InteractionFailure,
}

#[derive(Debug)]
pub enum SimulationError {
    SimulationTimeout,
    InvalidSimulationState,
}

#[derive(Debug)]
pub enum HashMismatch {
    HashesDoNotMatch,
}

#[derive(Debug)]
pub enum InvalidReference {
    ReferenceNotFound,
}

#[derive(Debug)]
pub enum SerializationError {
    SerializationFailed,
    DeserializationFailed,
}

#[derive(Debug)]
pub enum TemporalInconsistency {
    InconsistentTimeData,
}
