use crate::api::blockchain::ethereum::contract::Tokenizable;
use alloc::{format, vec::Vec};
use ethabi::Token;

/// Output type possible to deserialize from Contract ABI
pub trait Detokenize {
  /// Creates a new instance from parsed ABI tokens.
  fn from_tokens(tokens: Vec<Token>) -> crate::Result<Self>
  where
    Self: Sized;
}

impl<T> Detokenize for T
where
  T: Tokenizable,
{
  #[inline]
  fn from_tokens(mut tokens: Vec<Token>) -> crate::Result<Self> {
    let len = tokens.len();
    if let Some(first) = tokens.drain(..).next() {
      if len == 1 {
        return Self::from_token(first);
      }
    }
    Err(crate::Error::TokensInvalidOutputType(format!(
      "Expected single element, got a list of `{len}` elements",
    )))
  }
}

macro_rules! impl_tuples {
  ( $( ( $len:expr; $($ty: ident)+ ) )+ ) => {
    $(
      impl<$($ty, )+> Detokenize for ($($ty,)+)
      where
        $( $ty: Tokenizable, )+
      {
        #[inline]
        fn from_tokens(mut tokens: Vec<Token>) -> crate::Result<Self> {
          let len = tokens.len();
          let mut iter = tokens.drain(..);
          #[allow(non_snake_case)]
          if let ( $( Some($ty), )+ ) = ( $( iter.next().map(|elem| $ty::from_token(elem)), )+ ) {
            return Ok(( $( $ty?, )+ ));
          }
          Err(crate::Error::TokensInvalidOutputType(
            format!("Expected {} elements, got a list of {len} elements", $len))
          )
        }
      }
    )+
  }
}

impl_tuples!(
  (1usize;  A)
  (2usize;  A B)
  (3usize;  A B C)
  (4usize;  A B C D)
  (5usize;  A B C D E)
  (6usize;  A B C D E F)
  (7usize;  A B C D E F G)
  (8usize;  A B C D E F G H)
  (9usize;  A B C D E F G H I)
  (10usize; A B C D E F G H I J)
  (11usize; A B C D E F G H I J K)
  (12usize; A B C D E F G H I J K L)
  (13usize; A B C D E F G H I J K L M)
  (14usize; A B C D E F G H I J K L M N)
  (15usize; A B C D E F G H I J K L M N O)
  (16usize; A B C D E F G H I J K L M N O P)
);
