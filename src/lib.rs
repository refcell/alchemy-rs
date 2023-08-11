#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc=include_str!("../README.md")]

/// Refactored Websocket Connectors for the Alchemy Manager
pub mod connectors;

/// Refactored Alchemy Websocket Messages
pub mod messages;

/// Alchemy Manager
pub mod manager;

/// ethers-rs Alchemy wrappers
pub mod wrapper;

/// Common types
pub mod types;

/// A prelude of commonly used alchemy-rs items
pub mod prelude {
    pub use super::{manager::*, messages::prelude::*, types::*, wrapper::*};

    // Re-export ethers-rs prelude
    pub use ethers::prelude::*;
}
