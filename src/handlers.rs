//! Handler definitions and utilities

// Defines all handlers that are for Auth (BE) to App (BE) communication
pub mod auth_to_app;

// Defines all handlers that are for User (FE) to App (BE) communication
pub mod user_to_app;

// Defines some convenience utilities to prevent duplicated code on every app backend that uses the honey client
pub mod convenience_utils;
