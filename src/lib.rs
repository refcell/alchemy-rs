#![warn(missing_docs)]
#![warn(unused_extern_crates)]
#![forbid(unsafe_code)]
#![forbid(where_clauses_object_safety)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc=include_str!("../README.md")]

/// Alchemy Manager
pub mod manager;

/// ethers-rs Alchemy wrappers
pub mod wrapper;

/// Common types
pub mod types;

/// A prelude of commonly used alchemy-rs items
pub mod prelude {
    pub use super::{manager::*, types::*, wrapper::*};
}
