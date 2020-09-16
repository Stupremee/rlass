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

pub mod pool;
pub mod version;

/// The Magic number is used to
/// identify the `class` file format
pub const MAGIC: u32 = 0xCAFEBABE;

#[allow(non_camel_case_types)]
pub mod types {
    pub type u1 = u8;
    pub type u2 = u16;
    pub type u4 = u32;
}
