use crate::blockchain::solana::{SolanaSignatureHash, VersionedMessageInput};
use alloc::{vec, vec::Vec};
#[cfg(feature = "ed25519-dalek")]
use {
  crate::blockchain::solana::SolanaBlockhash,
  ed25519_dalek::{Keypair, Signer},
  lucia::misc::ByteBuffer,
};

/// Transport format suitable for user input.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInput {
  #[serde(with = "crate::blockchain::solana::short_vec")]
  /// Signatures
  pub signatures: Vec<SolanaSignatureHash>,
  /// Message
  pub message: VersionedMessageInput,
}

impl TransactionInput {
  /// Takes all the necessary parameters to validate and transform data into a suitable format for
  /// submission.
  #[cfg(feature = "ed25519-dalek")]
  pub fn new<'keypair, BB>(
    buffer: &mut BB,
    blockhash: SolanaBlockhash,
    message: VersionedMessageInput,
    keypairs: impl Clone + IntoIterator<Item = &'keypair Keypair>,
  ) -> crate::Result<Self>
  where
    BB: ByteBuffer,
  {
    let mut this = Self { signatures: <_>::default(), message };
    let VersionedMessageInput::V0(message) = &mut this.message;
    if blockhash != message.recent_blockhash {
      message.recent_blockhash = blockhash;
    }
    this._set_empty_signatures()?;
    this.do_sign(buffer, keypairs)?;
    Ok(this)
  }

  /// Checks if all signatures are actually signed.
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

  /// Signs or re-signs the contained message with the provided `blockhash` and `keypairs`.
  #[cfg(feature = "ed25519-dalek")]
  pub fn sign<'keypair, BB>(
    &mut self,
    blockhash: SolanaBlockhash,
    buffer: &mut BB,
    keypairs: impl Clone + IntoIterator<Item = &'keypair Keypair>,
  ) -> crate::Result<()>
  where
    BB: ByteBuffer,
  {
    let VersionedMessageInput::V0(message) = &mut self.message;
    if blockhash != message.recent_blockhash {
      message.recent_blockhash = blockhash;
      self.signatures.iter_mut().for_each(|signature| *signature = SolanaSignatureHash::default());
    }
    self.do_sign(buffer, keypairs)?;
    Ok(())
  }

  #[cfg(feature = "ed25519-dalek")]
  fn do_sign<'keypair, BB>(
    &mut self,
    mut buffer: &mut BB,
    keypairs: impl Clone + IntoIterator<Item = &'keypair Keypair>,
  ) -> crate::Result<()>
  where
    BB: ByteBuffer,
  {
    buffer.clear();
    bincode::serialize_into(&mut buffer, &self.message)?;
    let signing_keypair_positions = {
      let VersionedMessageInput::V0(message) = &self.message;
      let signed_keys = message
        .account_keys
        .get(0..message.header.num_required_signatures.into())
        .unwrap_or_default();
      keypairs.clone().into_iter().map(|keypair| {
        signed_keys.iter().position(|signed_key| keypair.public.as_bytes() == signed_key)
      })
    };
    for (opt, keypair) in signing_keypair_positions.zip(keypairs) {
      let signature = keypair.try_sign(buffer.as_ref())?.to_bytes();
      let signature_mut = match opt.and_then(|idx| self.signatures.get_mut(idx)) {
        None => {
          return Err(crate::Error::SolanaInexistentOrOutOfBoundsSignatureIndex(
            self.signatures.len(),
            opt,
          ));
        }
        Some(elem) => elem,
      };
      *signature_mut = signature.into();
    }
    self.check_signatures()?;
    buffer.clear();
    Ok(())
  }

  fn _set_empty_signatures(&mut self) -> crate::Result<()> {
    let VersionedMessageInput::V0(message) = &self.message;
    let len: usize = message.header.num_required_signatures.into();
    self.signatures = vec![SolanaSignatureHash::default(); len];
    Ok(())
  }
}
