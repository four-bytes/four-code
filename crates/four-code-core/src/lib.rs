//! four-code-core: Core editor functionality
//!
//! This crate provides the fundamental building blocks for the editor:
//! - Buffer: Text storage using rope data structure
//! - Cursor: Position and movement
//! - Selection: Range selections
//! - Editor: Combined state with viewport

mod buffer;
mod cursor;
mod editor;

pub use buffer::{Buffer, BufferError};
pub use cursor::{Cursor, Position};
pub use editor::{Editor, Viewport};

/// Re-export ropey for convenience
pub use ropey;
