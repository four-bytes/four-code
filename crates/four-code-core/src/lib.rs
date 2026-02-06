//! four-code-core: Core editor functionality
//!
//! This crate provides the fundamental building blocks for the editor:
//! - Buffer: Text storage using rope data structure
//! - Cursor: Position and movement
//! - Selection: Range selections

mod buffer;
mod cursor;

pub use buffer::Buffer;
pub use cursor::{Cursor, Position};

/// Re-export ropey for convenience
pub use ropey;
