//! Routes contains all of the endpoints that make up the TCIC Admin application

pub mod auth; // Authentication & Authorization, these mostly redirect after completing a task (or then end in failure, or it's a bot login)
pub mod core; // Normally accessed base web routes (anything too complex will get it's own module)
