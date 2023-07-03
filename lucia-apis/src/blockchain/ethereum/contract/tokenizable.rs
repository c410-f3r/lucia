use crate::blockchain::ethereum::{contract::TokenizableItem, Bytes};
use alloc::{format, string::String, vec::Vec};
use arrayvec::ArrayVec;
use ethabi::{Address, Token};
use ethereum_types::{H256, U128, U256};

/// Simplified output type for single value.
pub trait Tokenizable {
  /// Converts a `Token` into expected type.
  fn from_token(token: Token) -> crate::Result<Self>
  where
    Self: Sized;

  /// Converts a specified type back into token.
  fn into_token(self) -> Token;
}

impl<T, const N: usize> Tokenizable for [T; N]
where
  T: TokenizableItem,
{
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::FixedArray(tokens) = token {
      let len = tokens.len();
      let mut array = ArrayVec::<T, N>::new();
      for elem in tokens.into_iter().take(N).flat_map(T::from_token) {
        array.try_push(elem)?;
      }
      return array.into_inner().map_err(|_err| {
        crate::Error::TokensInvalidOutputType(format!(
          "Expected `FixedArray({N})`, got FixedArray({len})"
        ))
      });
    }
    Err(crate::Error::TokensInvalidOutputType(
      format!("Expected `FixedArray({N})`, got {token:?}",),
    ))
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::FixedArray(ArrayVec::from(self).into_iter().map(T::into_token).collect())
  }
}

impl<const N: usize> Tokenizable for [u8; N] {
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::FixedBytes(bytes) = token {
      if bytes.len() != N {
        return Err(crate::Error::TokensInvalidOutputType(format!(
          "Expected `FixedBytes({})`, got FixedBytes({})",
          N,
          bytes.len()
        )));
      }
      let mut arr = [0; N];
      arr.copy_from_slice(&bytes);
      return Ok(arr);
    }
    Err(crate::Error::TokensInvalidOutputType(format!(
      "Expected `FixedBytes({})`, got {:?}",
      N, token
    )))
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::FixedBytes(self.to_vec())
  }
}

impl Tokenizable for Address {
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::Address(data) = token {
      Ok(data)
    } else {
      Err(crate::Error::TokensInvalidOutputType(format!("Expected `Address`, got {:?}", token)))
    }
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::Address(self)
  }
}

impl Tokenizable for bool {
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::Bool(data) = token {
      Ok(data)
    } else {
      Err(crate::Error::TokensInvalidOutputType(format!("Expected `bool`, got {:?}", token)))
    }
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::Bool(self)
  }
}

impl Tokenizable for Bytes {
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::Bytes(s) = token {
      Ok(s.into())
    } else {
      Err(crate::Error::TokensInvalidOutputType(format!("Expected `Bytes`, got {:?}", token)))
    }
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::Bytes(self.0)
  }
}

impl Tokenizable for H256 {
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::FixedBytes(ref s) = token {
      if s.len() != 32 {
        let mut data = [0; 32];
        data.copy_from_slice(s);
        return Ok(data.into());
      }
    }
    Err(crate::Error::TokensInvalidOutputType(format!("Expected `H256`, got {:?}", token)))
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::FixedBytes(self.as_ref().to_vec())
  }
}

impl Tokenizable for String {
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::String(s) = token {
      Ok(s)
    } else {
      Err(crate::Error::TokensInvalidOutputType(format!("Expected `String`, got {:?}", token)))
    }
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::String(self)
  }
}

impl Tokenizable for Token {
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    Ok(token)
  }

  #[inline]
  fn into_token(self) -> Token {
    self
  }
}

impl<T> Tokenizable for Vec<T>
where
  T: TokenizableItem,
{
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::Array(tokens) | Token::FixedArray(tokens) = token {
      tokens.into_iter().map(Tokenizable::from_token).collect()
    } else {
      Err(crate::Error::TokensInvalidOutputType(format!("Expected `bytes`, got {:?}", token)))
    }
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::Array(self.into_iter().map(Tokenizable::into_token).collect())
  }
}

impl Tokenizable for Vec<u8> {
  #[inline]
  fn from_token(token: Token) -> crate::Result<Self> {
    if let Token::Bytes(data) | Token::FixedBytes(data) = token {
      Ok(data)
    } else {
      Err(crate::Error::TokensInvalidOutputType(format!("Expected `bytes`, got {:?}", token)))
    }
  }

  #[inline]
  fn into_token(self) -> Token {
    Token::Bytes(self)
  }
}

macro_rules! impl_unsigned_custom {
  ($uint:ident, $name:expr, $cb:expr) => {
    impl Tokenizable for $uint {
      #[inline]
      fn from_token(token: Token) -> crate::Result<Self> {
        match token {
          Token::Int(data) | Token::Uint(data) => {
            let cb: fn(U256) -> crate::Result<Self> = $cb;
            cb(data)
          }
          other => Err(crate::Error::TokensInvalidOutputType(format!(
            "Expected `{}`, got {:?}",
            $name, other
          ))),
        }
      }

      #[inline]
      fn into_token(self) -> Token {
        Token::Uint(self.into())
      }
    }
  };
}

impl_unsigned_custom!(U256, "U256", |data| Ok(data.into()));
impl_unsigned_custom!(U128, "U128", |data| Ok(data.try_into()?));

macro_rules! impl_unsigned_native {
  ($int:ident, $token:ident) => {
    impl Tokenizable for $int {
      #[inline]
      fn from_token(token: Token) -> crate::Result<Self> {
        match token {
          Token::Uint(data) => {
            data.try_into().map_err(|err: &'static str| crate::Error::Generic(err.into()))
          }
          other => Err(crate::Error::TokensInvalidOutputType(format!(
            "Expected `{}`, got {:?}",
            stringify!($int),
            other
          ))),
        }
      }

      #[inline]
      fn into_token(self) -> Token {
        Token::$token(self.into())
      }
    }
  };
}

impl_unsigned_native!(u8, Uint);
impl_unsigned_native!(u16, Uint);
impl_unsigned_native!(u32, Uint);
impl_unsigned_native!(u64, Uint);
impl_unsigned_native!(u128, Uint);
