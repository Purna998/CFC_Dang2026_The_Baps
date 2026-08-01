use anchor_lang::prelude::*;

#[error_code]
pub enum VotingError {
    #[msg("Program is paused by authority")]
    ProgramPaused,

    #[msg("Election has already been finalized")]
    ElectionFinalized,

    #[msg("Unauthorized: caller is not the election authority")]
    Unauthorized,

    #[msg("Invalid vote hash format")]
    InvalidVoteHash,

    #[msg("Invalid commitment signature")]
    InvalidCommitment,

    #[msg("Metadata exceeds maximum size of 128 bytes")]
    MetadataTooLarge,

    #[msg("Vote commitment already exists")]
    DuplicateCommitment,

    #[msg("Invalid election ID")]
    InvalidElectionId,

    #[msg("Invalid timestamp")]
    InvalidTimestamp,

    #[msg("Election not finalized yet")]
    ElectionNotFinalized,

    #[msg("Merkle root already generated")]
    MerkleRootExists,

    #[msg("No votes to generate merkle root")]
    NoVotesInElection,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Invalid tenant ID")]
    InvalidTenantId,
}
