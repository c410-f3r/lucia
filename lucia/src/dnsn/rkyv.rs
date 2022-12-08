use crate::misc::ByteBuffer;
use rkyv::{
  ser::{
    serializers::{
      AlignedSerializer, AllocScratch, CompositeSerializer, FallbackScratch, HeapScratch,
      SharedSerializeMap,
    },
    Serializer,
  },
  AlignedVec, Serialize,
};

pub(crate) type _InnerSerializer<'this> = CompositeSerializer<
  AlignedSerializer<&'this mut AlignedVec>,
  FallbackScratch<&'this mut HeapScratch<4096>, AllocScratch>,
  SharedSerializeMap,
>;

/// Type that indicates the usage of the `rkyv` dependency.
#[derive(Debug, Default)]
pub struct Rkyv(AlignedVec, HeapScratch<4096>);

impl Rkyv {
  pub(crate) fn _serialize<'this, BB, T>(
    &'this mut self,
    bytes: &mut BB,
    elem: &T,
  ) -> crate::Result<()>
  where
    BB: ByteBuffer,
    T: Serialize<_InnerSerializer<'this>>,
  {
    let mut serializer = _InnerSerializer::new(
      AlignedSerializer::new(&mut self.0),
      FallbackScratch::new(&mut self.1, AllocScratch::default()),
      SharedSerializeMap::default(),
    );
    let _ = serializer.serialize_value(elem)?;
    let aligned_vec = serializer.into_serializer().into_inner();
    bytes.extend(aligned_vec.iter().copied())?;
    aligned_vec.clear();
    Ok(())
  }
}
