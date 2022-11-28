use core::fmt::{Debug, Display};

/// This trait only exists because `trait_alias` is not stable.
pub trait DebugDisplay: Debug + Display {}

impl<T> DebugDisplay for T where T: Debug + Display {}
