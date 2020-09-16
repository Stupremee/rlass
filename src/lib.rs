#![forbid(
    unsafe_code,
    rust_2018_idioms,
    warnings,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::nursery,
    clippy::cargo
)]

pub mod types;
pub mod version;

/// The Magic number is used to
/// identify the `class` file format
pub const MAGIC: u32 = 0xCAFEBABE;
