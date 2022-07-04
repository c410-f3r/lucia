use crate::api::blockchain::solana::{
  short_vec, SolanaAddressHash, SolanaBlockhash, SolanaSignatureHash, MAX_BINARY_DATA_LEN,
  MAX_TRANSACTION_ACCOUNTS_NUM,
};
use alloc::{vec, vec::Vec};
use arrayvec::ArrayVec;

/// Compiled [InstructionInput]
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledInstructionInput {
  pub program_id_index: u8,
  #[serde(with = "short_vec")]
  pub accounts: ArrayVec<u8, MAX_TRANSACTION_ACCOUNTS_NUM>,
  #[serde(with = "short_vec")]
  pub data: ArrayVec<u8, MAX_BINARY_DATA_LEN>,
}

/// Used when performing requests
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionInput {
  pub accounts: ArrayVec<InstructionAccountInput, MAX_TRANSACTION_ACCOUNTS_NUM>,
  pub data: ArrayVec<u8, MAX_BINARY_DATA_LEN>,
  pub program_id: SolanaAddressHash,
}

#[cfg(feature = "solana-program")]
impl TryFrom<solana_program::instruction::Instruction> for InstructionInput {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: solana_program::instruction::Instruction) -> Result<Self, Self::Error> {
    Ok(Self {
      accounts: {
        let mut vec = ArrayVec::new();
        for elem in from.accounts {
          vec.try_push(elem.into())?;
        }
        vec
      },
      data: {
        let mut vec = ArrayVec::new();
        vec.try_extend_from_slice(&from.data)?;
        vec
      },
      program_id: from.program_id.to_bytes(),
    })
  }
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionAccountInput {
  pub pubkey: SolanaAddressHash,
  pub is_signer: bool,
  pub is_writable: bool,
}

#[cfg(feature = "solana-program")]
impl From<solana_program::instruction::AccountMeta> for InstructionAccountInput {
  #[inline]
  fn from(from: solana_program::instruction::AccountMeta) -> Self {
    Self {
      pubkey: from.pubkey.to_bytes(),
      is_signer: from.is_signer,
      is_writable: from.is_writable,
    }
  }
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInput {
  pub header: MessageHeaderInput,
  #[serde(with = "short_vec")]
  pub account_keys: Vec<SolanaAddressHash>,
  pub recent_blockhash: SolanaBlockhash,
  #[serde(with = "short_vec")]
  pub instructions: Vec<CompiledInstructionInput>,
}

impl MessageInput {
  #[inline]
  pub fn with_params(
    instructions: &[InstructionInput],
    payer: Option<SolanaAddressHash>,
    recent_blockhash: SolanaBlockhash,
  ) -> crate::Result<Self> {
    let (uniques, signers_num, readonly_signers_num, readonly_non_signers_num) =
      Self::unique_instructions(instructions, payer)?;
    Ok(Self {
      header: MessageHeaderInput {
        num_readonly_signed_accounts: readonly_signers_num,
        num_readonly_unsigned_accounts: readonly_non_signers_num,
        num_required_signatures: signers_num,
      },
      account_keys: {
        let mut vec = Vec::new();
        for unique in &uniques {
          vec.push(unique.pubkey);
        }
        vec
      },
      recent_blockhash,
      instructions: {
        let mut vec = Vec::new();
        for instruction in instructions {
          vec.push(Self::compile_instruction(instruction, &uniques)?);
        }
        vec
      },
    })
  }

  fn compile_instruction(
    instruction: &InstructionInput,
    uniques: &[InstructionAccountInput],
  ) -> crate::Result<CompiledInstructionInput> {
    let position = |public_key| {
      uniques
        .iter()
        .position(|elem| public_key == &elem.pubkey)
        .and_then(|elem| TryInto::<u8>::try_into(elem).ok())
        .ok_or(crate::Error::SolanaMessageCanNotHaveMoreThan240Accounts)
    };
    Ok(CompiledInstructionInput {
      program_id_index: position(&instruction.program_id)?,
      data: instruction.data.clone(),
      accounts: instruction
        .accounts
        .iter()
        .map(|elem| position(&elem.pubkey))
        .collect::<Result<_, _>>()?,
    })
  }

  fn unique_instructions(
    instructions: &[InstructionInput],
    payer: Option<SolanaAddressHash>,
  ) -> crate::Result<(Vec<InstructionAccountInput>, u8, u8, u8)> {
    let mut duplicates = Vec::new();

    let payer_instruction_account;
    if let Some(elem) = payer {
      payer_instruction_account =
        InstructionAccountInput { is_signer: true, is_writable: true, pubkey: elem };
      duplicates.push(payer_instruction_account);
    }

    for instruction in instructions.iter() {
      duplicates.push(InstructionAccountInput {
        is_signer: false,
        is_writable: false,
        pubkey: instruction.program_id,
      });
      for instruction_account in instruction.accounts.iter() {
        duplicates.push(InstructionAccountInput {
          is_signer: instruction_account.is_signer,
          is_writable: instruction_account.is_writable,
          pubkey: instruction_account.pubkey,
        });
      }
    }

    duplicates.sort_by(|first, second| {
      second.is_signer.cmp(&first.is_signer).then(second.is_writable.cmp(&first.is_writable))
    });

    let mut num_readonly_non_signer_accounts: u8 = 0;
    let mut num_readonly_signer_accounts: u8 = 0;
    let mut num_signer_accounts: u8 = 0;
    let mut uniques = Vec::<InstructionAccountInput>::new();
    for duplicate in duplicates {
      // Promote to writable if a later account requires it
      if let Some(elem) = uniques.iter_mut().find(|elem| elem.pubkey == duplicate.pubkey) {
        elem.is_writable |= duplicate.is_writable;
        continue;
      }
      let instruction_account = InstructionAccountInput {
        is_signer: duplicate.is_signer,
        is_writable: duplicate.is_writable,
        pubkey: duplicate.pubkey,
      };
      if instruction_account.is_signer {
        num_signer_accounts = num_signer_accounts.wrapping_add(1);
        if !instruction_account.is_writable {
          num_readonly_signer_accounts = num_readonly_signer_accounts.wrapping_add(1);
        }
      } else if !instruction_account.is_writable {
        num_readonly_non_signer_accounts = num_readonly_non_signer_accounts.wrapping_add(1);
      } else {
      }
      uniques.push(instruction_account);
    }

    if uniques.len() > MAX_TRANSACTION_ACCOUNTS_NUM {
      return Err(crate::Error::SolanaMessageCanNotHaveMoreThan240Accounts);
    }

    Ok((
      uniques,
      num_signer_accounts,
      num_readonly_signer_accounts,
      num_readonly_non_signer_accounts,
    ))
  }
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageHeaderInput {
  pub num_required_signatures: u8,
  pub num_readonly_signed_accounts: u8,
  pub num_readonly_unsigned_accounts: u8,
}

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInput {
  #[serde(with = "short_vec")]
  pub signatures: Vec<SolanaSignatureHash>,
  pub message: MessageInput,
}

impl TransactionInput {
  #[cfg(feature = "std")]
  #[inline]
  pub fn new<'keypair, B>(
    mut buffer: &mut B,
    blockhash: SolanaBlockhash,
    message: MessageInput,
    keypairs: impl Clone + IntoIterator<Item = &'keypair ed25519_dalek::Keypair>,
  ) -> crate::Result<Self>
  where
    B: AsRef<[u8]> + std::io::Write,
  {
    use ed25519_dalek::Signer;
    let mut this = Self { signatures: Default::default(), message };
    this._set_empty_signatures()?;
    if blockhash != this.message.recent_blockhash {
      this.message.recent_blockhash = blockhash;
    }
    let signed_keys = this
      .message
      .account_keys
      .get(..this.message.header.num_required_signatures.into())
      .unwrap_or_default();
    let signing_keypair_positions = keypairs.clone().into_iter().map(|keypair| {
      signed_keys.iter().position(|signed_key| keypair.public.as_bytes() == signed_key)
    });
    bincode::serialize_into(&mut buffer, &this.message)?;
    for (opt, keypair) in signing_keypair_positions.zip(keypairs) {
      let signature = keypair.try_sign(buffer.as_ref())?.to_bytes();
      let signature_mut = match opt.and_then(|idx| this.signatures.get_mut(idx)) {
        None => {
          return Err(crate::Error::SolanaInexistentOrOutOfBoundsSignatureIndex(
            this.signatures.len(),
            opt,
          ));
        }
        Some(elem) => elem,
      };
      *signature_mut = signature.into();
    }
    this.check_signatures()?;
    Ok(this)
  }

  #[inline]
  pub fn check_signatures(&self) -> crate::Result<()> {
    let default = SolanaSignatureHash::default();
    let mut filled: usize = 0;
    let all_are_signed = self.signatures.iter().all(|signature| {
      filled = filled.wrapping_add(1);
      signature != &default
    });
    if all_are_signed {
      Ok(())
    } else {
      let len = self.signatures.len();
      Err(crate::Error::SolanaSignersShouldHaveSignedAllTransactionSignatures(filled, len))
    }
  }

  fn _set_empty_signatures(&mut self) -> crate::Result<()> {
    let len: usize = self.message.header.num_required_signatures.into();
    self.signatures = vec![SolanaSignatureHash::default(); len];
    Ok(())
  }
}
