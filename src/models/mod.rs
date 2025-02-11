//! # Models
//!
//! This module defines the data structures (models or entities) that represent the core concepts
//! of the application's domain. These models are database-agnostic and primarily focus on
//! data representation.
//!
//! They are used to define the shape of the data as it exists within the application,
//! independent of how it's stored or retrieved.
//!
//! Example:
//!
//! ```
//! // A simple model for a Subscriber
//! #[derive(Debug, Clone)]
//! pub struct Subscriber {
//!     pub id: i32,
//!     pub email: String,
//!     pub name: String,
//! }
//! ```
