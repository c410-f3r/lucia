use crate::blockchain::ethereum::contract::Tokenizable;
use alloc::{string::String, vec::Vec};
use ethabi::{Address, Token};
use ethereum_types::{H256, U128, U256};

/// Marker trait for `Tokenizable` types that are can tokenized to and from a
/// `Token::Array` and `Token:FixedArray`.
pub trait TokenizableItem: Tokenizable {}

impl<const N: usize> TokenizableItem for [u8; N] {}

impl<T, const N: usize> TokenizableItem for [T; N] where T: TokenizableItem {}

impl<T> TokenizableItem for Vec<T> where T: TokenizableItem {}

macro_rules! impl_tokenizable {
  ($( $type:ty ),+) => {
    $( impl TokenizableItem for $type {} )+
  };
}

impl_tokenizable!(Address, bool, H256, String, Token, u128, U128, u16, U256, u32, u64, Vec<u8>);
