#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod blocking;
pub mod error;
pub mod nonblocking;
mod protocol;

pub use blocking::*;
pub use protocol::Timestamp;
