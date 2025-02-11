//! # Domain
//!
//! This module defines the core business logic and data structures of the application,
//! independent of any specific implementation details such as databases or external APIs.
//!
//! It contains value objects, entities, and domain services that represent the fundamental
//! concepts and rules of the application's problem domain.
//!
//! Example:
//!
//! ```
//! // A value object representing an email address
//! #[derive(Debug, Clone)]
//! pub struct EmailAddress(pub String);
//!
//! // An entity representing a User
//! pub struct User {
//!     pub id: UserId,
//!     pub email: EmailAddress,
//!     pub name: String,
//! }
//!
//! pub struct UserId(pub String);
//! ```
