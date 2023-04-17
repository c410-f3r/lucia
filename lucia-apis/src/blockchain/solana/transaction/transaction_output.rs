use crate::blockchain::solana::{
  program::spl_token::AccountBalance, CompiledInstructionJson, InnerInstructionJson,
  InstructionJsonParsedGeneric, MessageJsonAccountKey, SolanaAddressHashStr, TransactionEncoding,
  TransactionJson, TransactionVersion,
};
use alloc::{string::String, vec::Vec};

/// A transaction can be represented in various formats.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum GenericTransaction {
  /// Encoded bytes.
  Base64(String, TransactionEncoding),
  /// Decoded bytes as JSON.
  Json(TransactionJson<SolanaAddressHashStr, CompiledInstructionJson>),
  /// Decoded bytes as JSON with additional metadata.
  JsonParsed(TransactionJson<MessageJsonAccountKey, InstructionJsonParsedGeneric>),
}

/// An instruction wasn't successful.
#[allow(
  // Probably little will be gained boxing a variant of 28 bytes
  variant_size_differences
)]
#[derive(Debug, serde::Deserialize)]
pub enum InstructionError {
  /// Deprecated! Use CustomError instead!
  /// The program instruction returned an error
  GenericError,

  /// The arguments provided to a program were invalid
  InvalidArgument,

  /// An instruction's data contents were invalid
  InvalidInstructionData,

  /// An account's data contents was invalid
  InvalidAccountData,

  /// An account's data was too small
  AccountDataTooSmall,

  /// An account's balance was too small to complete the instruction
  InsufficientFunds,

  /// The account did not have the expected program id
  IncorrectProgramId,

  /// A signature was required but not found
  MissingRequiredSignature,

  /// An initialize instruction was sent to an account that has already been initialized.
  AccountAlreadyInitialized,

  /// An attempt to operate on an account that hasn't been initialized.
  UninitializedAccount,

  /// Program's instruction lamport balance does not equal the balance after the instruction
  UnbalancedInstruction,

  /// Program illegally modified an account's program id
  ModifiedProgramId,

  /// Program spent the lamports of an account that doesn't belong to it
  ExternalAccountLamportSpend,

  /// Program modified the data of an account that doesn't belong to it
  ExternalAccountDataModified,

  /// Read-only account's lamports modified
  ReadonlyLamportChange,

  /// Read-only account's data was modified
  ReadonlyDataModified,

  /// An account was referenced more than once in a single instruction
  // Deprecated, instructions can now contain duplicate accounts
  DuplicateAccountIndex,

  /// Executable bit on account changed, but shouldn't have
  ExecutableModified,

  /// Rent_epoch account changed, but shouldn't have
  RentEpochModified,

  /// The instruction expected additional account keys
  NotEnoughAccountKeys,

  /// Program other than the account's owner changed the size of the account data
  AccountDataSizeChanged,

  /// The instruction expected an executable account
  AccountNotExecutable,

  /// Failed to borrow a reference to account data, already borrowed
  AccountBorrowFailed,

  /// Account data has an outstanding reference after a program's execution
  AccountBorrowOutstanding,

  /// The same account was multiply passed to an on-chain program's entrypoint, but the program
  /// modified them differently.  A program can only modify one instance of the account because
  /// the runtime cannot determine which changes to pick or how to merge them if both are modified
  DuplicateAccountOutOfSync,

  /// Allows on-chain programs to implement program-specific error types and see them returned
  /// by the Solana runtime. A program-specific error may be any type that is represented as
  /// or serialized to a u32 integer.
  Custom(u32),

  /// The return value from the program was invalid.  Valid errors are either a defined builtin
  /// error value or a user-defined error in the lower 32 bits.
  InvalidError,

  /// Executable account's data was modified
  ExecutableDataModified,

  /// Executable account's lamports modified
  ExecutableLamportChange,

  /// Executable accounts must be rent exempt
  ExecutableAccountNotRentExempt,

  /// Unsupported program id
  UnsupportedProgramId,

  /// Cross-program invocation call depth too deep
  CallDepth,

  /// An account required by the instruction is missing
  MissingAccount,

  /// Cross-program invocation reentrancy not allowed for this instruction
  ReentrancyNotAllowed,

  /// Length of the seed is too long for address generation
  MaxSeedLengthExceeded,

  /// Provided seeds do not result in a valid address
  InvalidSeeds,

  /// Failed to reallocate account data of this length
  InvalidRealloc,

  /// Computational budget exceeded
  ComputationalBudgetExceeded,

  /// Cross-program invocation with unauthorized signer or writable account
  PrivilegeEscalation,

  /// Failed to create program execution environment
  ProgramEnvironmentSetupFailure,

  /// Program failed to complete
  ProgramFailedToComplete,

  /// Program failed to compile
  ProgramFailedToCompile,

  /// Account is immutable
  Immutable,

  /// Incorrect authority provided
  IncorrectAuthority,

  /// Failed to serialize or deserialize account data
  ///
  /// Warning: This error should never be emitted by the runtime.
  ///
  /// This error includes strings from the underlying 3rd party Borsh crate
  /// which can be dangerous because the error strings could change across
  /// Borsh versions. Only programs can use this error because they are
  /// consistent across Solana software versions.
  BorshIoError(String),

  /// An account does not have enough lamports to be rent-exempt
  AccountNotRentExempt,

  /// Invalid account owner
  InvalidAccountOwner,

  /// Program arithmetic overflowed
  ArithmeticOverflow,

  /// Unsupported sysvar
  UnsupportedSysvar,

  /// Illegal account owner
  IllegalOwner,
  // Note: For any new error added here an equivalent ProgramError and its
  // conversions must also be added
}

/// Some possible reasons a transaction was rejected.
#[derive(Debug, serde::Deserialize)]
pub enum TransactionError {
  /// An account is already being processed in another transaction in a way
  /// that does not support parallelism
  AccountInUse,

  /// A `Pubkey` appears twice in the transaction's `account_keys`.  Instructions can reference
  /// `Pubkey`s more than once but the message must contain a list with no duplicate keys
  AccountLoadedTwice,

  /// Attempt to debit an account but found no record of a prior credit.
  AccountNotFound,

  /// Attempt to load a program that does not exist
  ProgramAccountNotFound,

  /// The from `Pubkey` does not have sufficient balance to pay the fee to schedule the transaction
  InsufficientFundsForFee,

  /// This account may not be used to pay transaction fees
  InvalidAccountForFee,

  /// The bank has seen this transaction before. This can occur under normal operation
  /// when a UDP packet is duplicated, as a user error from a client not updating
  /// its `recent_blockhash`, or as a double-spend attack.
  AlreadyProcessed,

  /// The bank has not seen the given `recent_blockhash` or the transaction is too old and
  /// the `recent_blockhash` has been discarded.
  BlockhashNotFound,

  /// An error occurred while processing an instruction. The first element of the tuple
  /// indicates the instruction index in which the error occurred.
  InstructionError(u8, InstructionError),

  /// Loader call chain is too deep
  CallChainTooDeep,

  /// Transaction requires a fee but has no signature present
  MissingSignatureForFee,

  /// Transaction contains an invalid account reference
  InvalidAccountIndex,

  /// Transaction did not pass signature verification
  SignatureFailure,

  /// This program may not be used for executing instructions
  InvalidProgramForExecution,

  /// Transaction failed to sanitize accounts offsets correctly
  /// implies that account locks are not taken for this TX, and should
  /// not be unlocked.
  SanitizeFailure,

  /// Cluster is in maintenance state.
  ClusterMaintenance,

  /// Transaction processing left an account with an outstanding borrowed reference
  AccountBorrowOutstanding,

  /// Transaction would exceed max Block Cost Limit
  WouldExceedMaxBlockCostLimit,

  /// Transaction version is unsupported
  UnsupportedVersion,

  /// Transaction loads a writable account that cannot be written
  InvalidWritableAccount,

  /// Transaction would exceed max account limit within the block
  WouldExceedMaxAccountCostLimit,

  /// Transaction would exceed max account data limit within the block
  WouldExceedMaxAccountDataCostLimit,
}

/// Transaction metadata.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionMeta {
  /// Error filled if unsuccessful.
  pub err: Option<TransactionError>,
  /// Lamports changed by this transaction;
  pub fee: u64,
  /// List of inner instructions or null if inner instruction recording was not enabled during
  /// this transaction.
  pub inner_instructions: Vec<InnerInstructionJson>,
  /// Lost of string log messages or null if log message recording was not enabled during this
  /// transaction.
  pub log_messages: Option<Vec<String>>,
  /// Debited or credited lamports of all accounts before applying the transaction.
  pub post_balances: Vec<u64>,
  /// Debited or credited tokens of all accounts before applying the transaction.
  pub post_token_balances: Option<Vec<TransactionTokenBalance>>,
  /// Debited or credited lamports of all accounts after applying the transaction.
  pub pre_balances: Vec<u64>,
  /// Debited or credited tokens of all accounts after applying the transaction.
  pub pre_token_balances: Option<Vec<TransactionTokenBalance>>,
}

/// Transaction output of a RPC request.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionOutput {
  /// Estimated production time, as Unix timestamp
  pub block_time: Option<i64>,
  /// Transaction metadata
  pub meta: Option<TransactionMeta>,
  /// The slot this transaction was processed in.
  pub slot: u64,
  /// Generic transaction
  pub transaction: GenericTransaction,
  /// Transaction version
  pub version: Option<TransactionVersion>,
}

/// Pre and pos transaction balance information.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionTokenBalance {
  /// Index in regards to the block array of accounts.
  pub account_index: u8,
  /// Base58 identifier.
  pub mint: SolanaAddressHashStr,
  /// Base58 identifier.
  pub owner: SolanaAddressHashStr,
  /// Token balance.
  pub ui_token_amount: AccountBalance,
}
