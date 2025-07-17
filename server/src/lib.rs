//! Main library module for the TagBlaze server.
//!
//! This module re-exports submodules for routing, database operations, request handlers,
//! configuration management, data models, and utility functions.
//!
//! # Modules
//! - `routes`: Defines API endpoints and routing logic.
//! - `db`: Handles database connections and queries.
//! - `handlers`: Contains request handler implementations.
//! - `config`: Manages application configuration and environment variables.
//! - `models`: Defines data structures and ORM models.
//! - `utils`: Provides utility functions used throughout the server.
pub mod config;
pub mod db;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;
