use std::fmt::{Display, Formatter, Error};

/// The Magic number is used to
/// identify the `class` file format
const MAGIC: u32 = 0xCAFEBABE;

/// The values of the minor_version and major_version items are the
/// minor and major version numbers of this class file.
/// Together, a major and a minor version number determine the version
/// of the class file format.
/// If a class file has major version number M and minor version number m,
/// we denote the version of its class file format as M.m.
/// Thus, class file format versions may be ordered lexicographically, for example, 1.5 < 2.0 < 2.1.
///
/// # ClassFile version table
///
/// |   Java SE |   Supported versions      |
/// | :-------: |   :-------------------:   |
/// |   1.0.2   |   45.0 <= v <= 45.3       |
/// |   1.1     |   45.0 <= v <= 45.65535   |
/// |   1.2     |   45.0 <= v <= 46.0       |
/// |   1.3     |   45.0 <= v <= 47.0       |
/// |   1.4     |   45.0 <= v <= 48.0       |
/// |   5.0     |   45.0 <= v <= 49.0       |
/// |   6       |   45.0 <= v <= 50.0       |
/// |   7       |   45.0 <= v <= 51.0       |
/// |   8       |   45.0 <= v <= 52.0       |
/// |   9       |   45.0 <= v <= 53.0       |
/// |   10      |   45.0 <= v <= 54.0       |
/// |   11      |   45.0 <= v <= 55.0       |
///
/// # Important
/// At the moment rlass only supports class file version up to 55.0.
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClassFileVersion {
    /// The major number of this version
    pub major: u16,
    /// The minor number of this version
    pub minor: u16,
}

impl ClassFileVersion {
    /// Returns a new ClassFileVersion with the given major and minor version.
    ///
    /// # Arguments
    ///
    /// * `major` - The major version of this class file version
    /// * `minor` - The minor version of this class file version
    ///
    /// # Example
    ///
    /// ```
    /// // This snippet creates a new ClassFileVersion which represents the version 55.0
    /// use rlass::class_file::ClassFileVersion;
    /// let version = ClassFileVersion::new(55, 0);
    /// ```
    pub fn new(major: u16, minor: u16) -> Self {
        ClassFileVersion { minor, major }
    }

    /// Returns a new ClassFileVersion which represents the latest supported class file version.
    /// At the moment the latest supported version is `55.0`.
    pub fn latest() -> Self {
        Self::new(55, 0)
    }
}

impl Display for ClassFileVersion {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}.{}", self.major, self.minor)
    }
}