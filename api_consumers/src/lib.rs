//! Any part of these applications that touches an outside source (e.g. API) will go through this library
//! This is mostly to separate someone else's problems from my own problems.

pub mod epic; // Epic Game Store API routes
pub mod oeis; // OEIS routes
pub mod ollama; // LLM routes
pub mod openweathermap; // Weather routes
pub mod twitch; // Twitch API routes
pub mod wikipedia; // Wikipedia
pub mod words; // Dick Tionary and The Saurus
pub mod youtube; // White videos
