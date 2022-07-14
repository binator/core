#![doc = include_str!("../readme.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::missing_const_for_fn)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_like_matches_macro)]
#![feature(try_trait_v2)]
#![warn(missing_docs)]
#![deny(clippy::default_numeric_fallback)]
// #![feature(never_type)]
// #![feature(exhaustive_patterns)]
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::fmt::{
  Debug,
  Display,
  Formatter,
};

mod contexting;
pub use contexting::*;
mod parse;
pub use parse::*;
mod success;
pub use success::*;
mod parsed_aux;
pub use parsed_aux::*;
mod parsed;
pub use parsed::*;

mod streaming;
pub use streaming::*;

mod acc;
pub use acc::*;
mod try_acc;
pub use try_acc::*;
mod extend;
pub use extend::*;
mod try_extend;
pub use try_extend::*;
mod push;
pub use push::*;
mod try_push;
pub use try_push::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Core context used to implement context for basic type like u8
pub enum CoreAtom<Stream, Error = <Stream as Streaming>::Error> {
  /// Used when end of stream is reached.
  EndOfStream {
    /// The stream that returned end of stream.
    stream: Stream,
  },
  /// Used when stream return an Error.
  Error {
    /// the error returned by the stream.
    error: Error,
  },
}

impl<Stream: Streaming> Display for CoreAtom<Stream>
where
  Stream: Debug,
  Stream::Error: Debug,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::EndOfStream { .. } => write!(f, "End of stream"),
      Self::Error { error } => write!(f, "The stream have encounter an error {:?}", error,),
    }
  }
}
