//! Hanzo Tools - The unified tool registry for all Hanzo services
//! 
//! One and exactly one way to do everything. No modes, no alternatives.
//! Just import `hanzo_tools::Registry` and you have all tools.
//!
//! # Example
//! ```rust
//! use hanzo_tools::Registry;
//! 
//! let registry = Registry::new();
//! let tools = registry.list_tools();
//! println!("Available tools: {}", tools.len());
//! ```

mod registry;

// Re-export everything from registry at the crate root
pub use registry::{ToolRegistry, ToolInfo, ToolError, ToolResult};

// Convenience aliases
pub type Registry = ToolRegistry;
pub type Tools = ToolRegistry;