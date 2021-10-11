#![recursion_limit = "100"]
//! The network-agnostic DNS parser library
//!
//! [Documentation](https://docs.rs/dns-parser) |
//! [Github](https://github.com/tailhook/dns-parser) |
//! [Crate](https://crates.io/crates/dns-parser)
//!
//! Use [`Builder`] to create a new outgoing packet.
//!
//! Use [`Packet::parse`] to parse a packet into a data structure.
//!
//! [`Builder`]: struct.Builder.html
//! [`Packet::parse`]: struct.Packet.html#method.parse
//!
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

#[cfg(test)]
#[macro_use]
extern crate matches;
#[macro_use(quick_error)]
extern crate quick_error;

mod builder;
mod enums;
mod error;
mod header;
mod name;
mod parser;
mod structs;

/// Data types and methods for handling the RData field
#[allow(missing_docs)] // resource records are pretty self-descriptive
pub mod rdata;

pub use crate::builder::Builder;
pub use crate::enums::{Class, Opcode, QueryClass, ResponseCode};
pub use crate::error::Error;
pub use crate::header::Header;
pub use crate::name::Name;
pub use crate::rdata::{QueryType, RData, Type};
pub use crate::structs::{Packet, Question, ResourceRecord};
