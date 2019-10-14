/// Representation of all constant pool
/// [tags](https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.4-140).
#[derive(Debug, PartialEq)]
pub enum Tag { Class,
    FieldRef,
    MethodRef,
    InterfaceMethodRef,
    String,
    Integer,
    Float,
    Long,
    Double,
    NameAndType,
    Utf8,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
    // Invalid tag is required to be returned in
    // the From<u8> implementation if the given tag is
    // invalid.
    Invalid,
}

impl From<u8> for Tag {
    fn from(tag: u8) -> Self {
        match tag {
            7 => Tag::Class,
            9 => Tag::FieldRef,
            10 => Tag::MethodRef,
            11 => Tag::InterfaceMethodRef,
            8 => Tag::String,
            3 => Tag::Integer,
            4 => Tag::Float,
            5 => Tag::Long,
            6 => Tag::Double,
            12 => Tag::NameAndType,
            1 => Tag::Utf8,
            15 => Tag::MethodHandle,
            16 => Tag::MethodType,
            17 => Tag::Dynamic,
            18 => Tag::InvokeDynamic,
            19 => Tag::Module,
            20 => Tag::Package,
            _ => Tag::Invalid,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ReferenceKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
    // Invalid tag is required to be returned in
    // the From<u8> implementation if the given tag is
    // invalid.
    Invalid,
}

impl From<u8> for ReferenceKind {
    fn from(kind: u8) -> Self {
        match kind {
            1 => ReferenceKind::GetField,
            2 => ReferenceKind::GetStatic,
            3 => ReferenceKind::PutField,
            4 => ReferenceKind::PutStatic,
            5 => ReferenceKind::InvokeVirtual,
            6 => ReferenceKind::InvokeStatic,
            7 => ReferenceKind::InvokeSpecial,
            8 => ReferenceKind::NewInvokeSpecial,
            9 => ReferenceKind::InvokeInterface,
            _ => ReferenceKind::Invalid,
        }
    }
}
