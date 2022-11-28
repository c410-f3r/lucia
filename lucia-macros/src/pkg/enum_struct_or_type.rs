use syn::{ItemEnum, ItemStruct, ItemType};

#[derive(Clone, Copy, Debug)]
pub(crate) enum EnumStructOrType<'any> {
  Enum(&'any ItemEnum),
  Struct(&'any ItemStruct),
  Type(&'any ItemType),
}
