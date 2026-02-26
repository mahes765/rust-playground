pub mod repo;
pub mod branch;
pub mod commit;
pub mod status;

// Re-export commonly used functions
pub use repo::open_repo;
pub use repo::get_current_branch;