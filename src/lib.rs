#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod postgres;
pub mod query_functions;
pub mod themes;
pub mod ui;
pub use app::Rosemary;
