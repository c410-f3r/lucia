use borsh::BorshSerialize;

use crate::misc::ByteBuffer;

/// Type that indicates the usage of the `borsh` dependency.
#[derive(Debug, Default)]
pub struct Borsh;

impl Borsh {
  pub(crate) fn _serialize<'this, BB, T>(
    &'this mut self,
    bytes: &mut BB,
    elem: &T,
  ) -> crate::Result<()>
  where
    BB: ByteBuffer,
    T: BorshSerialize,
  {
    elem.serialize(bytes)?;
    Ok(())
  }
}
