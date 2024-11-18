use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Type {
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

impl Type {
    pub fn new_void() -> Type {
        Type {
            size: 0,
            align: 0,
            kind: TypeKind::Void,
        }
    }

    pub fn new_bool() -> Type {
        Type {
            size: 1,
            align: 1,
            kind: TypeKind::Bool,
        }
    }

    pub fn new_char() -> Type {
        Type {
            size: 1,
            align: 1,
            kind: TypeKind::Char,
        }
    }

    pub fn new_int() -> Type {
        Type {
            size: 4,
            align: 4,
            kind: TypeKind::Int,
        }
    }

    pub fn new_ptr(t: Type) -> Type {
        Type {
            size: 8,
            align: 8,
            kind: TypeKind::Ptr(Box::new(t)),
        }
    }

    pub fn new_array(t: Type, len: u32) -> Type {
        Type {
            size: t.size * len,
            align: t.align,
            kind: TypeKind::Array(Box::new(t), len),
        }
    }

    pub fn new_func(t: Type, params: Vec<Type>) -> Type {
        Type {
            size: 0,
            align: 0,
            kind: TypeKind::Func(Box::new(t), params),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    ty: Type,
    name: String,
    kind: VarKind,
}

#[derive(Debug, Clone)]
enum VarKind {
    Local(u32),
    Global(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    kind: NodeKind,
    ty: Type,
    name: String,
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Num(u32),
    VarDef(Var),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Eq(Box<Node>, Box<Node>),
    Ne(Box<Node>, Box<Node>),
    Lt(Box<Node>, Box<Node>),
    Le(Box<Node>, Box<Node>),
    Assign(Box<Node>, Box<Node>),
    Addr(Box<Node>),
    Deref(Box<Node>),
    Return(Box<Node>),
    If(Box<Node>, Box<Node>, Box<Node>),
    For(Box<Node>, Box<Node>, Box<Node>, Box<Node>),
    Block(Vec<Node>),
    FuncCall(String, Vec<Node>),
}
