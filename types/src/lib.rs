//! Contains all of the independent types in this project
//! There are some types/structs in other places in the project, esith in `local_types` for dependent types
//! or they're where they are due to the choices of dependencies, e.g. diesel schema/models in `database`
#![warn(missing_debug_implementations)]

pub mod auth;
pub mod database;
pub mod get;
pub mod twitch;
