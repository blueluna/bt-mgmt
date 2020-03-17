#[macro_use] extern crate bitflags;

pub mod eir;
pub mod error;
pub mod events;
#[macro_use] mod extended_enum;
pub mod pack;
mod socket;
mod system;

pub use socket::{Event, Socket};
pub use error::Error;