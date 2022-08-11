macro_rules! _create_set_of_request_throttling {
  (
    $name:ident {
      $( $method:ident ),+ $(,)?
    }
  ) => {
    /// A set of [$crate::utils::RequestThrottling] for specified API usage
    #[derive(Debug)]
    pub struct $name {
      $(
        pub(crate) $method: $crate::utils::RequestThrottling,
      )+
    }

    impl $name {
      #[inline]
      pub fn new(
        $( $method: $crate::utils::RequestLimit, )+
      ) -> Self {
        Self {
          $(
            $method: $crate::utils::RequestThrottling::from_rl($method),
          )+
        }
      }
    }
  };
}

macro_rules! _debug {
  ($($tt:tt)+) => {
    #[cfg(feature = "tracing")]
    tracing::debug!($($tt)+);
  };
}

macro_rules! generic_data_format_doc {
  ($ty:literal) => {
    concat!(
      "Internal wrapper used in every generic ",
      $ty,
      " to manage different internal implementations."
    )
  };
}

// Implements `Serialize` for several collections
//
// Must be in sync with `requests.rs`.
macro_rules! _impl_se_collections {
  (
    for $drsr:ty => $bound:path;

    $( array: |$array_self:ident, $array_bytes:ident, $array_drsr:ident| $array_block:block )?
    $( arrayvec: |$arrayvec_self:ident, $arrayvec_bytes:ident, $arrayvec_drsr:ident| $arrayvec_block:block )?
    slice_ref: |$slice_ref_self:ident, $slice_ref_bytes:ident, $slice_ref_drsr:ident| $slice_ref_block:block
    vec: |$vec_self:ident, $vec_bytes:ident, $vec_drsr:ident| $vec_block:block
  ) => {
    $(
      impl<T, const N: usize> crate::dnsn::Serialize<$drsr> for [T; N]
      where
        T: $bound,
      {
        #[inline]
        fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
        where
          B: crate::misc::ByteBuffer,
        {
          let $array_self = self;
          let $array_bytes = bytes;
          let $array_drsr = drsr;
          $array_block;
          Ok(())
        }
      }
    )?

    $(
      impl<T, const N: usize> crate::dnsn::Serialize<$drsr> for arrayvec::ArrayVec<T, N>
      where
        T: $bound,
      {
        #[inline]
        fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
        where
          B: crate::misc::ByteBuffer,
        {
          let $arrayvec_self = self;
          let $arrayvec_bytes = bytes;
          let $arrayvec_drsr = drsr;
          $arrayvec_block;
          Ok(())
        }
      }
    )?

    impl<T> crate::dnsn::Serialize<$drsr> for &'_ [T]
    where
      T: $bound,
    {
      #[inline]
      fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
      where
        B: crate::misc::ByteBuffer,
      {
        let $slice_ref_self = self;
        let $slice_ref_bytes = bytes;
        let $slice_ref_drsr = drsr;
        $slice_ref_block;
        Ok(())
      }
    }

    impl<T> crate::dnsn::Serialize<$drsr> for Vec<T>
    where
      T: $bound,
    {
      #[inline]
      fn to_bytes<B>(&self, bytes: &mut B, drsr: &mut $drsr) -> crate::Result<()>
      where
        B: crate::misc::ByteBuffer,
      {
        let $vec_self = self;
        let $vec_bytes = bytes;
        let $vec_drsr = drsr;
        $vec_block;
        Ok(())
      }
    }
  };
}
