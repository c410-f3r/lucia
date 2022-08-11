/// Any contiguous block of memory that can be dynamically extended.
pub trait Buffer<T>:
  AsMut<[T]>
  + cl_aux::Clear
  + cl_aux::Extend<T, Error = cl_aux::Error, Output = ()>
  + cl_aux::Push<T, Error = cl_aux::Error, Output = ()>
{
}

impl<T, U> Buffer<T> for U where
  U: AsMut<[T]>
    + cl_aux::Clear
    + cl_aux::Extend<T, Error = cl_aux::Error, Output = ()>
    + cl_aux::Push<T, Error = cl_aux::Error, Output = ()>
{
}

macro_rules! create_byte_buffer {
  ($($bounds:tt)*) => {
    /// Specialized [Buffer] trait for bytes.
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
create_byte_buffer!(Buffer<u8> + std::io::Write);
#[cfg(not(feature = "std"))]
create_byte_buffer!(Buffer<u8>);
