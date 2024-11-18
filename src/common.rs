use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Type {
    size: u32,
    align: u32,
    kind: TypeKind,
}

#[derive(Debug, Clone)]
enum TypeKind {
    Void,
    Bool,
    Char,
    Int,
    Ptr(Box<Type>),
    Array(Box<Type>, u32),
    // Struct(Vec<Field>),
    Func(Box<Type>, Vec<Type>),
}
