/// Marker that imposes `Send + Sync` bounds if the `async-trait` feature is used.
#[cfg(not(feature = "async-trait"))]
pub trait AsyncTrait {}

#[cfg(not(feature = "async-trait"))]
impl<T> AsyncTrait for T where T: ?Sized {}

/// Marker that imposes `Send + Sync` bounds if the `async-trait` feature is used.
#[cfg(feature = "async-trait")]
pub trait AsyncTrait: Send + Sync {}

#[cfg(feature = "async-trait")]
impl<T> AsyncTrait for T where T: Send + Sync + ?Sized {}
