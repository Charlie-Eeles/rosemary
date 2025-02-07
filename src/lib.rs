#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub mod postgres;
pub use app::Rosemary;
