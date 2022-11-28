use crate::blockchain::solana::{
  SolanaAddressHash, SolanaBlockhash, SolanaSignatureHash, MAX_TRANSACTION_ACCOUNTS_NUM,
};
use alloc::{vec, vec::Vec};

/// Compiled [InstructionInput]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct CompiledInstructionInput {
  pub program_id_index: u8,
  #[cfg_attr(feature = "serde", serde(with = "crate::blockchain::solana::short_vec"))]
  pub accounts: Vec<u8>,
  #[cfg_attr(feature = "serde", serde(with = "crate::blockchain::solana::short_vec"))]
  pub data: Vec<u8>,
}

/// Used when performing requests
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct InstructionInput {
  pub accounts: Vec<InstructionAccountInput>,
  pub data: Vec<u8>,
  pub program_id: SolanaAddressHash,
}

#[cfg(feature = "solana-program")]
impl TryFrom<solana_program::instruction::Instruction> for InstructionInput {
  type Error = crate::Error;

  #[inline]
  fn try_from(from: solana_program::instruction::Instruction) -> Result<Self, Self::Error> {
    Ok(Self {
      accounts: from.accounts.into_iter().map(|elem| elem.into()).collect(),
      data: from.data,
      program_id: from.program_id.to_bytes(),
    })
  }
}

#[cfg(feature = "solana-program")]
impl From<InstructionInput> for solana_program::instruction::Instruction {
  #[inline]
  fn from(from: InstructionInput) -> Self {
    Self {
      accounts: from.accounts.into_iter().map(|elem| elem.into()).collect(),
      data: from.data.into_iter().map(|elem| elem.into()).collect(),
      program_id: from.program_id.into(),
    }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct InstructionAccountInput {
  pub pubkey: SolanaAddressHash,
  pub is_signer: bool,
  pub is_writable: bool,
}

impl InstructionAccountInput {
  pub fn none(pubkey: SolanaAddressHash) -> Self {
    Self { pubkey, is_signer: false, is_writable: false }
  }

  pub fn sign(pubkey: SolanaAddressHash) -> Self {
    Self { pubkey, is_signer: true, is_writable: false }
  }

  pub fn sign_and_write(pubkey: SolanaAddressHash) -> Self {
    Self { pubkey, is_signer: true, is_writable: true }
  }

  pub fn write(pubkey: SolanaAddressHash) -> Self {
    Self { pubkey, is_signer: false, is_writable: true }
  }
}

#[cfg(feature = "solana-program")]
impl From<solana_program::instruction::AccountMeta> for InstructionAccountInput {
  #[inline]
  fn from(from: solana_program::instruction::AccountMeta) -> Self {
    Self {
      is_signer: from.is_signer,
      is_writable: from.is_writable,
      pubkey: from.pubkey.to_bytes(),
    }
  }
}

#[cfg(feature = "solana-program")]
impl From<InstructionAccountInput> for solana_program::instruction::AccountMeta {
  #[inline]
  fn from(from: InstructionAccountInput) -> Self {
    Self { is_signer: from.is_signer, is_writable: from.is_writable, pubkey: from.pubkey.into() }
  }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct MessageInput {
  pub header: MessageHeaderInput,
  #[cfg_attr(feature = "serde", serde(with = "crate::blockchain::solana::short_vec"))]
  pub account_keys: Vec<SolanaAddressHash>,
  pub recent_blockhash: SolanaBlockhash,
  #[cfg_attr(feature = "serde", serde(with = "crate::blockchain::solana::short_vec"))]
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

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct MessageHeaderInput {
  pub num_required_signatures: u8,
  pub num_readonly_signed_accounts: u8,
  pub num_readonly_unsigned_accounts: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[derive(Debug)]
pub struct TransactionInput {
  #[cfg_attr(feature = "serde", serde(with = "crate::blockchain::solana::short_vec"))]
  pub signatures: Vec<SolanaSignatureHash>,
  pub message: MessageInput,
}

impl TransactionInput {
  #[cfg(feature = "ed25519-dalek")]
  #[inline]
  pub fn new<'keypair, BB>(
    mut buffer: &mut BB,
    blockhash: SolanaBlockhash,
    message: MessageInput,
    keypairs: impl Clone + IntoIterator<Item = &'keypair ed25519_dalek::Keypair>,
  ) -> crate::Result<Self>
  where
    BB: lucia::misc::ByteBuffer,
  {
    use ed25519_dalek::Signer;
    let mut this = Self { signatures: <_>::default(), message };
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
    buffer.clear();
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
    buffer.clear();
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
