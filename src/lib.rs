#![forbid(
    unsafe_code,
    rust_2018_idioms,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::nursery,
    clippy::cargo
)]

use std::io::{self, Read};

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

pub type Result<T, E = Error> = std::result::Result<T, E>;

use byteorder::{ReadBytesExt, BE};
use pool::{ConstantPoolEntry, UnlinkedConstantPoolEntry, UnlinkedRefEntry};
use thiserror::Error;
use types::*;

#[derive(Debug, Error)]
pub enum Error {
    #[error("a constant pool entry contains an invalid index")]
    InvalidIndex,
    #[error("the class file contains an invalid magic number")]
    InvalidMagic,
    #[error("{0}")]
    Io(#[from] io::Error),
}

/// The representation of a class file, that is emitted
/// by javac.
///
/// An instance of a `ClassFile` must be obtained by
/// using the [`ClassParser`]
#[derive(Debug, Clone)]
pub struct ClassFile {
    pub version: version::ClassFileVersion,
    pub constant_pool: Vec<pool::ConstantPoolEntry>,
}

/// The `ClassLoader` is responsible for
/// parsing a class file.
pub struct ClassParser<R> {
    inner: R,
}

impl<R> ClassParser<R> {
    /// Creates a new `ClassParser` with the given inner read.
    pub fn new(inner: R) -> Self {
        Self { inner }
    }

    /// Returns the underlying inner value of this `ClassParser`.
    pub fn into_inner(self) -> R {
        self.inner
    }
}

impl<R: Read> ClassParser<R> {
    /// Tries to parse a `ClassFile` from the inner reader of
    /// this `ClassParser`.
    pub fn parse(&mut self) -> Result<ClassFile> {
        let magic = self.read_u4()?;
        if magic != MAGIC {
            return Err(Error::InvalidMagic);
        }

        let major_version = self.read_u2()?;
        let minor_version = self.read_u2()?;
        let version = version::ClassFileVersion::new(major_version, minor_version);

        let cp_len = self.read_u2()?;
        let mut constant_pool = Vec::new();
        for _ in 0..cp_len {
            let tag = self.read_u1()?;
            let entry = match tag {
                // MethodRef
                10 => {
                    let class = self.read_u2()?;
                    let name_and_ty = self.read_u2()?;
                    UnlinkedConstantPoolEntry::MethodRef(UnlinkedRefEntry { class, name_and_ty })
                }
                tag => todo!("constant pool tag {}", tag),
            };

            constant_pool.push(entry);
        }

        let constant_pool = self.link_constant_pool(constant_pool)?;
        Ok(ClassFile {
            version,
            constant_pool,
        })
    }

    fn link_constant_pool(
        &self,
        cp: Vec<UnlinkedConstantPoolEntry>,
    ) -> Result<Vec<ConstantPoolEntry>> {
        todo!()
    }

    /// Reads a single byte from the inner reader.
    fn read_u1(&mut self) -> io::Result<u1> {
        let mut buf = [0u8; 1];
        let read = self.inner.read(&mut buf)?;
        debug_assert_eq!(read, 1);
        Ok(buf[0])
    }

    /// Reads a two byte, big-endian encoded integer.
    fn read_u2(&mut self) -> io::Result<u2> {
        self.inner.read_u16::<BE>()
    }

    /// Reads a four byte, big-endian encoded integer.
    fn read_u4(&mut self) -> io::Result<u4> {
        self.inner.read_u32::<BE>()
    }
}
