use crate::blockchain::ethereum::contract::Tokenizable;
use alloc::{vec, vec::Vec};
use ethabi::Token;

/// Tokens conversion trait
pub trait Tokenize {
  /// Convert to list of tokens
  fn into_tokens(self) -> Vec<Token>;
}

impl Tokenize for () {
  #[inline]
  fn into_tokens(self) -> Vec<Token> {
    Vec::new()
  }
}

impl Tokenize for &'_ [Token] {
  #[inline]
  fn into_tokens(self) -> Vec<Token> {
    self.to_vec()
  }
}

impl<T> Tokenize for T
where
  T: Tokenizable,
{
  #[inline]
  fn into_tokens(self) -> Vec<Token> {
    vec![self.into_token()]
  }
}

macro_rules! impl_tuples {
  (
    $( ( $( $ty:ident : $idx:tt )+ ) )+
  ) => {
    $(
      impl<$($ty),+> Tokenize for ($($ty,)+)
      where
        $( $ty: Tokenizable ),+
      {
        #[inline]
        fn into_tokens(self) -> Vec<Token> {
          vec![
            $( self.$idx.into_token() ),+
          ]
        }
      }
    )+
  }
}

impl_tuples!(
  (A:0)
  (A:0 B:1)
  (A:0 B:1 C:2)
  (A:0 B:1 C:2 D:3)
  (A:0 B:1 C:2 D:3 E:4)
  (A:0 B:1 C:2 D:3 E:4 F:5)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7 I:8)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7 I:8 J:9)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7 I:8 J:9 K:10)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7 I:8 J:9 K:10 L:11)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7 I:8 J:9 K:10 L:11 M:12)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7 I:8 J:9 K:10 L:11 M:12 N:13)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7 I:8 J:9 K:10 L:11 M:12 N:13 O:14)
  (A:0 B:1 C:2 D:3 E:4 F:5 G:6 H:7 I:8 J:9 K:10 L:11 M:12 N:13 O:14 P:15)
);
