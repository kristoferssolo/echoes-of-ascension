//! # Repositories
//!
//! This module provides an abstraction layer for data access. Repositories encapsulate the
//! logic for interacting with the database, providing a consistent interface for creating,
//! reading, updating, and deleting data.
//!
//! They are responsible for translating between the application's domain models and the
//! database's data structures.  The database connection pool is passed as a parameter to
//! the repository methods.
//!
//! Example:
//!
//! ```rust
//! use sqlx::PgPool;
//! use sqlx::Error;
//!
//! // A repository for managing Subscribers
//! #[derive(Debug, Clone, Copy)]
//! pub struct SubscriberRepository;
//!
//! pub async fn create_subscriber(pool: &PgPool, email: &str, name: &str) -> Result<(), Error> {
//!     // ... database interaction logic using the pool
//!     Ok(())
//! }
//! ```

pub mod user;
