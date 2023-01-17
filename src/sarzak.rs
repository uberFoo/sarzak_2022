//! Sarzak Domain
//!
//! This file was generated by: `sarzak new "sarzak"`.
use uuid::{uuid, Uuid};

#[macro_use]
pub mod macros;
pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// sarzak
pub const UUID_NS: Uuid = uuid!("daccabb9-eb3a-5cde-ba7c-19a3f22ab649");
