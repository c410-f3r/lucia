macro_rules! create_byte_buffer {
  ($($bounds:tt)*) => {
    /// Specialized [cl_aux::DynContigColl] trait for bytes.
    pub trait ByteBuffer: $($bounds)* {
    }

    impl<U> ByteBuffer for U
    where
      U: $($bounds)*
    {
    }
  };
}

#[cfg(feature = "std")]
create_byte_buffer!(cl_aux::DynContigColl<u8> + std::io::Write);
#[cfg(not(feature = "std"))]
create_byte_buffer!(cl_aux::DynContigColl<u8>);
