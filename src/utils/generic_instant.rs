use core::time::Duration;

/// Tries to support different time machinery on different platforms.
///
/// Currently only supports `std`. For anything else, methods are no-op.
#[derive(Clone, Copy, Debug)]
pub struct GenericInstant {
  #[cfg(feature = "std")]
  inner: std::time::Instant,
}

impl GenericInstant {
  /// Returns an instant corresponding to "now".
  #[inline]
  pub fn now() -> Self {
    Self {
      #[cfg(feature = "std")]
      inner: std::time::Instant::now(),
    }
  }

  /// Returns the amount of time elapsed from another instant to this one,
  /// or None if that instant is later than this one.
  #[inline]
  pub fn checked_duration_since(&self, _earlier: Self) -> Option<Duration> {
    #[cfg(feature = "std")]
    {
      self.inner.checked_duration_since(_earlier.inner)
    }
    #[cfg(not(feature = "std"))]
    {
      None
    }
  }

  /// Returns the amount of time elapsed since this instant was created.
  #[inline]
  pub fn elapsed(&self) -> Duration {
    #[cfg(feature = "std")]
    {
      self.inner.elapsed()
    }
    #[cfg(not(feature = "std"))]
    {
      <_>::default()
    }
  }
}
