use crate::{utils::log, Request, Requests};
use core::fmt::Debug;
use serde::Deserialize;

macro_rules! tuple_impls {
  ($(
    $tuple_len:tt {
      $(($idx:tt) -> $T:ident)*
    }
  )+) => {
    $(
      /// The JSON-RPC 2.0 specification does not enforce the order of the server
      /// response so beware.
      impl<$( $T, )*> Requests<()> for ($( $T, )*)
      where
      $(
          $T: Debug + Request + serde::Serialize,
          $T::RawResponse: Debug + for<'de> Deserialize<'de>,
        )*
      {
        type Output = ($( $T::RawResponse, )*);

        #[inline]
        fn manage_responses(&self, _: &mut (), bytes: &[u8]) -> crate::Result<Self::Output> {
          log(&self, bytes);
          let responses: Self::Output = serde_json::from_slice(bytes)?;
          Ok(responses)
        }
      }
    )+
  }
}

tuple_impls! {
  0 {
  }
  1 {
    (0) -> A
  }
  2 {
    (0) -> A
    (1) -> B
  }
  3 {
    (0) -> A
    (1) -> B
    (2) -> C
  }
  4 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
  }
  5 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
  }
  6 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
  }
  7 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
  }
  8 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
  }
  9 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
  }
  10 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
  }
  11 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
    (10) -> K
  }
  12 {
    (0) -> A
    (1) -> B
    (2) -> C
    (3) -> D
    (4) -> E
    (5) -> F
    (6) -> G
    (7) -> H
    (8) -> I
    (9) -> J
    (10) -> K
    (11) -> L
  }
}
