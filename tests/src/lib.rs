#![allow(unused_imports)]

use force_derive::*;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};
use std::marker::PhantomData;

#[derive(Debug)]
struct DebugOnly;

#[derive(Debug, ForceDefault, ForceClone, ForceCopy, ForceEq, ForcePartialEq, ForceHash)]
pub struct TupleStruct<T>(u32, PhantomData<T>);

#[cfg(test)]
impl<T> TupleStruct<T> {
    fn new(value: u32) -> Self {
        Self(value, PhantomData)
    }
}

#[test]
fn tuple_struct_default() {
    assert_eq!(0, TupleStruct::<DebugOnly>::default().0);
}

#[test]
fn tuple_struct_eq() {
    assert_eq!(TupleStruct::<DebugOnly>::default(), TupleStruct::default());
    assert_ne!(TupleStruct::<DebugOnly>::default(), TupleStruct::new(1));
}

#[test]
fn tuple_struct_hash() {
    let mut set = std::collections::HashSet::<TupleStruct<DebugOnly>>::default();
    assert!(set.insert(TupleStruct::default()));
    assert!(set.insert(TupleStruct::new(1)));
    assert!(!set.insert(TupleStruct::default()));
}

#[test]
fn tuple_struct_copy() {
    let value = TupleStruct::<DebugOnly>::default();
    let copy = value;
    drop(value);
    drop(copy);
}

#[derive(Debug, ForceDefault, ForceClone, ForceCopy, ForceEq, ForcePartialEq, ForceHash)]
pub struct FieldStruct<T> {
    value: u32,
    marker: PhantomData<T>,
}

#[cfg(test)]
impl<T> FieldStruct<T> {
    fn new(value: u32) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

#[test]
fn field_struct_default() {
    assert_eq!(0, FieldStruct::<DebugOnly>::default().value);
}

#[test]
fn field_struct_eq() {
    assert_eq!(FieldStruct::<DebugOnly>::default(), FieldStruct::default());
    assert_ne!(FieldStruct::<DebugOnly>::default(), FieldStruct::new(1));
}

#[test]
fn field_struct_hash() {
    let mut set = std::collections::HashSet::<FieldStruct<DebugOnly>>::default();
    assert!(set.insert(FieldStruct::default()));
    assert!(set.insert(FieldStruct::new(1)));
    assert!(!set.insert(FieldStruct::default()));
}

#[test]
fn field_struct_copy() {
    let value = FieldStruct::<DebugOnly>::default();
    let copy = value;
    drop(value);
    drop(copy);
}

#[derive(Debug, ForceDefault, ForceClone, ForceCopy, ForceEq, ForcePartialEq, ForceHash)]
pub struct UnitStruct;

#[test]
fn unit_struct_default() {
    UnitStruct::default();
}

#[test]
fn unit_struct_eq() {
    assert_eq!(UnitStruct::default(), UnitStruct::default());
}

#[test]
fn unit_struct_hash() {
    let mut set = std::collections::HashSet::<UnitStruct>::default();
    assert!(set.insert(UnitStruct::default()));
    assert!(!set.insert(UnitStruct::default()));
}

#[test]
fn unit_struct_copy() {
    let value = UnitStruct::default();
    let copy = value;
    drop(value);
    drop(copy);
}

#[derive(Debug, ForceDefault, ForceCopy, ForceClone, ForceEq, ForcePartialEq, ForceHash)]
pub enum NamedEnum<T> {
    First { a: u32 },
    Second { value: PhantomData<T> },
}

impl<T> NamedEnum<T> {
    pub fn first(a: u32) -> Self {
        Self::First { a }
    }
}

#[test]
fn named_enum_default() {
    NamedEnum::<DebugOnly>::default();
}

#[test]
fn named_enum_clone() {
    let named_enum = NamedEnum::<DebugOnly>::first(1);
    let _clone = named_enum.clone();
}

#[test]
fn named_enum_copy() {
    let named_enum = NamedEnum::<DebugOnly>::first(1);
    let copy = named_enum;
    drop(named_enum);
    drop(copy);
}

#[test]
fn named_enum_partial_eq() {
    assert_eq!(
        NamedEnum::<DebugOnly>::First { a: 0 },
        NamedEnum::First { a: 0 }
    );
    assert_ne!(
        NamedEnum::<DebugOnly>::First { a: 0 },
        NamedEnum::First { a: 1 }
    );
    assert_ne!(
        NamedEnum::<DebugOnly>::Second { value: PhantomData },
        NamedEnum::First { a: 0 }
    );
    assert_eq!(
        NamedEnum::<DebugOnly>::Second { value: PhantomData },
        NamedEnum::Second { value: PhantomData },
    );
}

#[test]
fn named_enum_hash() {
    let s = &RandomState::new();

    assert_eq!(
        get_hash(NamedEnum::<DebugOnly>::First { a: 0 }, s),
        get_hash(NamedEnum::<DebugOnly>::First { a: 0 }, s)
    );
    assert_ne!(
        get_hash(NamedEnum::<DebugOnly>::First { a: 0 }, s),
        get_hash(NamedEnum::<DebugOnly>::First { a: 1 }, s)
    );
    assert_eq!(
        get_hash(NamedEnum::<DebugOnly>::Second { value: PhantomData }, s),
        get_hash(NamedEnum::<DebugOnly>::Second { value: PhantomData }, s)
    );
}

#[derive(Debug, ForceDefault, ForceCopy, ForceClone, ForceEq, ForcePartialEq, ForceHash)]
pub enum UnnamedEnum<T> {
    First(u32, u64),
    Second(PhantomData<T>),
}

#[test]
fn unnamed_enum_default() {
    UnnamedEnum::<DebugOnly>::default();
}

#[test]
fn unnamed_enum_clone() {
    let unnamed_enum = UnnamedEnum::<DebugOnly>::First(1, 2);
    let _clone = unnamed_enum.clone();
}

#[test]
fn unnamed_enum_copy() {
    let unnamed_enum = UnnamedEnum::<DebugOnly>::First(1, 2);
    let copy = unnamed_enum;
    drop(unnamed_enum);
    drop(copy);
}

#[test]
fn unnamed_enum_partial_eq() {
    assert_eq!(
        UnnamedEnum::<DebugOnly>::First(1, 2),
        UnnamedEnum::First(1, 2)
    );
    assert_ne!(
        UnnamedEnum::<DebugOnly>::First(1, 2),
        UnnamedEnum::First(2, 2)
    );
    assert_ne!(
        UnnamedEnum::<DebugOnly>::First(1, 2),
        UnnamedEnum::First(1, 1)
    );
    assert_ne!(
        UnnamedEnum::<DebugOnly>::First(1, 2),
        UnnamedEnum::Second(PhantomData)
    );
    assert_eq!(
        UnnamedEnum::<DebugOnly>::Second(PhantomData),
        UnnamedEnum::Second(PhantomData)
    );
}

#[test]
fn unnamed_enum_hash() {
    let s = &RandomState::new();

    assert_eq!(
        get_hash(UnnamedEnum::<DebugOnly>::First(1, 2), s),
        get_hash(UnnamedEnum::<DebugOnly>::First(1, 2), s)
    );
    assert_ne!(
        get_hash(UnnamedEnum::<DebugOnly>::First(1, 2), s),
        get_hash(UnnamedEnum::<DebugOnly>::First(2, 2), s)
    );
    assert_ne!(
        get_hash(UnnamedEnum::<DebugOnly>::First(1, 2), s),
        get_hash(UnnamedEnum::<DebugOnly>::First(1, 1), s)
    );
    assert_ne!(
        get_hash(UnnamedEnum::<DebugOnly>::First(1, 2), s),
        get_hash(UnnamedEnum::<DebugOnly>::Second(PhantomData), s)
    );
    assert_eq!(
        get_hash(UnnamedEnum::<DebugOnly>::Second(PhantomData), s),
        get_hash(UnnamedEnum::<DebugOnly>::Second(PhantomData), s)
    );
}

#[derive(Debug, ForceDefault, ForceCopy, ForceClone, ForcePartialEq, ForceHash)]
pub enum UnitEnum {
    First,
    Second,
}

#[test]
fn unit_enum_default() {
    UnitEnum::default();
}

#[test]
fn unit_enum_clone() {
    let unit_enum = UnitEnum::First;
    let _clone = unit_enum.clone();
}

#[test]
fn unit_enum_copy() {
    let unit_enum = UnitEnum::First;
    let copy = unit_enum;
    drop(unit_enum);
    drop(copy);
}

#[test]
fn unit_enum_eq() {
    assert_eq!(UnitEnum::First, UnitEnum::First);
    assert_eq!(UnitEnum::Second, UnitEnum::Second);
    assert_ne!(UnitEnum::First, UnitEnum::Second);
}

#[test]
fn unit_enum_hash() {
    let s = &RandomState::new();

    assert_eq!(get_hash(UnitEnum::First, s), get_hash(UnitEnum::First, s));
    assert_ne!(get_hash(UnitEnum::First, s), get_hash(UnitEnum::Second, s));
    assert_eq!(get_hash(UnitEnum::Second, s), get_hash(UnitEnum::Second, s));
}

#[cfg(test)]
fn get_hash<H: Hash>(value: H, s: &RandomState) -> u64 {
    let mut hasher = s.build_hasher();
    value.hash(&mut hasher);
    hasher.finish()
}
