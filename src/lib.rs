//! An NTP client library for use with and without the standard library.
//!
//! This library providers both block and non-blocking (async) interfaces for getting the
//! current time from an NTP server.
//!
//! The `async` feature must be enabled to use the async interface found in
//! [`nonblocking`](nonblocking).
//!
//! The `std` feature can be enabled for an implementation of [`NtpTransport`](NtpTransport)
//! and [`NtpTransportAsync`](nonblocking::NtpTransportAsync) that uses the standard library's `std::net::UdpSocket`.
//!
//! The `chrono` feature can be enabled for an implementation of [`From<Timestamp>`](protocol::Timestamp::from)
//! to the [`chrono`](https://crates.io/crates/chrono) crate's `NaiveDateTime` and `DateTime<Utc>` types.
//!
//! In order to use the library you will first need an implementation of the [`NtpTransport`](NtpTransport)
//! or [`NtpTransportAsync`](nonblocking::NtpTransportAsync) trait.
//!
//! Then you can use one of [`sntp_get_transmit_timestamp`](sntp_get_transmit_timestamp) or
//! [`sntp_get_transmit_timestamp`](nonblocking::sntp_get_transmit_timestamp) to get the current time from
//! an NTP server. DNS lookup functionality is not provided by this library.

#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod blocking;
pub mod error;
pub mod nonblocking;
mod protocol;

pub use blocking::*;
pub use protocol::Timestamp;
