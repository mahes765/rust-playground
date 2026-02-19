pub mod repo;
pub mod branch;
pub mod commit;
pub mod status;

// Re-export git::open_repo()
pub use repo::open_repo;