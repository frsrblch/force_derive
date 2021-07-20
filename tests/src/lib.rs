use force_derive::*;
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

#[derive(Debug, ForceDefault, ForceCopy, ForceClone)]
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

#[derive(Debug, ForceDefault, ForceCopy, ForceClone)]
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

#[derive(Debug, ForceDefault, ForceCopy, ForceClone)]
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
