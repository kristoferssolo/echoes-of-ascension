//! # Services
//!
//! This module contains the business logic of the application. Services orchestrate the
//! interaction between models, repositories, and other components to fulfill specific use
//! cases.
//!
//! They define the core operations that the application performs and are responsible for
//! enforcing business rules and ensuring data consistency.
//!
//! Example:
//!
//! ```
//! // A service for managing newsletter subscriptions
//! pub struct NewsletterService {
//!     // ... dependencies (e.g., SubscriberRepository, EmailClient)
//! }
//!
//! impl NewsletterService {
//!     // Method to subscribe a user to the newsletter
//!     pub async fn subscribe_user(&self, email: &str, name: &str) -> Result<(), String> {
//!         // ... business logic (e.g., validate email, create subscriber, send confirmation email)
//!         Ok(())
//!     }
//! }
//! ```
