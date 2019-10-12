#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(String),
    String(String),
    // FieldRef, MethodRef, etc
    NameAndType(TypeAndName),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAndName {
    /// Represents either a valid unqualified name denoting a field or method, or the special method name <init> (§2.9.1).
    pub name: String,
    /// Represents a valid field descriptor or method descriptor.
    pub descriptor: String,
}