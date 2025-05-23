#![no_std]

#[repr(u16)]
pub enum U8ToU16Enum {
    Bar,
    Baz,
}

#[repr(i8)]
pub enum I32ToI8Enum {
    Bar,
    Baz,
}

#[repr(u32)]
pub enum I32ToU32Enum {
    Bar,
    Baz,
}

#[repr(C, u16)]
pub enum CU8ToCU16Enum {
    Bar,
    Baz,
}


// The following enums have changes that can be breaking on some systems.
// Since there are no guarantees on what particular system a given crate
// might run on, this is a breaking change.

#[repr(usize)]
pub enum U64ToUsizeEnum {
    Bar,
    Baz,
}

#[repr(u64)]
pub enum UsizeToU64Enum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum I64ToIsizeEnum {
    Bar,
    Baz,
}

#[repr(i64)]
pub enum IsizeToI64Enum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum UsizeToIsizeEnum {
    Bar,
    Baz,
}

#[repr(usize)]
pub enum IsizeToUsizeEnum {
    Bar,
    Baz,
}


// The following enums have *removals* of repr(i*) and repr(u*),
// not changes to another repr(i*) or repr(u*).
// They should not be reported by this rule, because they have their own rule.


pub enum U8EnumToEnum {
    Bar,
    Baz,
}

pub enum I32EnumToEnum {
    Bar,
    Baz,
}

pub enum IsizeEnumToEnum {
    Bar,
    Baz,
}

pub enum UsizeEnumToEnum {
    Bar,
    Baz,
}

#[repr(C)]
pub enum CU8EnumToCEnum {
    Bar,
    Baz,
}
