//! Implementation of the constant pool specification.
//!
//! See [here] for more information.
//!
//! [here]: https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.4

use crate::types::u2;
use std::rc::Rc;

/// A single, unlinked entry in the constant pool.
///
/// This is only used during parsing, and will be "linked"
/// later to a read `ConstantPoolEntry`.
/// This entry uses raw indexes in the constant pool instead of
/// `Rc<Entry>` indicies.
#[derive(Debug, Clone)]
pub(crate) enum UnlinkedConstantPoolEntry {
    Class(u2),
    FieldRef(UnlinkedRefEntry),
    MethodRef(UnlinkedRefEntry),
    InterfaceMethodRef(UnlinkedRefEntry),
    String(u2),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType {
        name: u2,
        descriptor: u2,
    },
    Utf8(String),
    MethodHandle {
        ref_kind: MethodHandleKind,
        ref_entry: u2,
    },
    MethodType(u2),
    /// bootstrap_attr_idx and name_and_type index
    Dynamic(u2, u2),
    /// bootstrap_attr_idx and name_and_type index
    InvokeDynamic(u2, u2),
    Module(u2),
    Package(u2),
}

#[derive(Debug, Clone)]
pub(crate) struct UnlinkedRefEntry {
    pub(crate) class: u2,
    pub(crate) name_and_ty: u2,
}

/// A single entry in the constant pool.
///
/// See [here](https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.4) for
/// more information.
#[derive(Debug, Clone)]
pub enum ConstantPoolEntry {
    Class(Rc<Utf8Entry>),
    FieldRef(RefEntry),
    MethodRef(RefEntry),
    InterfaceMethodRef(RefEntry),
    String(Rc<Utf8Entry>),
    Integer(ValEntry<i32>),
    Float(ValEntry<f32>),
    Long(ValEntry<i64>),
    Double(ValEntry<f64>),
    NameAndType(Rc<NameAndTypeEntry>),
    Utf8(Rc<Utf8Entry>),
    MethodHandle(MethodHandleEntry),
    MethodType(MethodTypeEntry),
    Dynamic(DynamicEntry),
    InvokeDynamic(DynamicEntry),
    Module(Rc<Utf8Entry>),
    Package(Rc<Utf8Entry>),
}

/// `CONSTANT_Dynamic_info` entry in the constant pool.
///
/// See [here].
///
/// [here]: https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-4.html#jvms-4.4.10
#[derive(Debug, Clone)]
pub struct DynamicEntry {
    /// Index into the `bootstrap_methods` of a class file.
    pub bootstrap_attr_idx: usize,
    pub name_and_ty: Rc<NameAndTypeEntry>,
}

/// A entry to a literal generic value.
#[derive(Debug, Clone)]
pub struct ValEntry<T> {
    pub val: T,
}

/// A Utf8 entry in the constant pool.
#[derive(Debug, Clone)]
pub struct Utf8Entry {
    pub bytes: String,
}

/// A `NameAndType` entry in the constant pool.
#[derive(Debug, Clone)]
pub struct NameAndTypeEntry {
    pub name: Rc<Utf8Entry>,
    pub descriptor: Rc<Utf8Entry>,
}

/// This entry is used for `FieldRef`, `MethodRef`, and `InterfaceMethodRef`.
#[derive(Debug, Clone)]
pub struct RefEntry {
    pub class: Rc<Utf8Entry>,
    pub name_and_ty: Rc<NameAndTypeEntry>,
}

/// A `MethodHandle` entry in the constant pool.
#[derive(Debug, Clone)]
pub struct MethodHandleEntry {
    pub ref_kind: MethodHandleKind,
    pub ref_entry: Rc<RefEntry>,
}

/// A `MethodType` entry in the constant pool.
#[derive(Debug, Clone)]
pub struct MethodTypeEntry {
    pub descriptor: Rc<Utf8Entry>,
}

/// The kind of a `MethodHandle` entry in the constant pool.
///
/// See [here](https://docs.oracle.com/javase/specs/jvms/se11/html/jvms-5.html#jvms-5.4.3.5) for
/// more information.
#[derive(Debug, Clone)]
pub enum MethodHandleKind {
    GetField,
    GetStatic,
    PutField,
    PutStatic,
    InvokeVirtual,
    InvokeStatic,
    InvokeSpecial,
    NewInvokeSpecial,
    InvokeInterface,
}
