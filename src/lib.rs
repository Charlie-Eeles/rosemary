#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod postgres;
pub mod queries;
pub mod ui;
pub use app::Rosemary;
