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

impl Var {
    pub fn new(ty: Type, name: String) -> Var {
        Var {
            ty,
            name,
            kind: VarKind::Local(0),
        }
    }

    // pub fn new_global(ty: Type, name: String) -> Var {
    //     Var {
    //         ty: ty,
    //         name: name,
    //         kind: VarKind::Global(name),
    //     }
    // }
}

#[derive(Debug, Clone)]
enum VarKind {
    Local(u32),
    Global(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub ty: Type,
    pub name: String,
}

impl Node {
    pub fn new_defalut() -> Node {
        Node {
            kind: NodeKind::Num(0),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NodeKind {
    Num(u32),
    VarDef(Box<Node>),
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

impl Node {
    pub fn new_num(val: u32) -> Node {
        Node {
            kind: NodeKind::Num(val),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }

    pub fn new_var_def(ty: Type, name: String, init: Node) -> Node {
        Node {
            kind: NodeKind::VarDef(Box::new(init)),
            ty,
            name,
        }
    }
    pub fn new_add(lhs: Node, rhs: Node) -> Node {
        Node {
            kind: NodeKind::Add(Box::new(lhs), Box::new(rhs)),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }

    pub fn new_sub(lhs: Node, rhs: Node) -> Node {
        Node {
            kind: NodeKind::Sub(Box::new(lhs), Box::new(rhs)),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }

    pub fn new_mul(lhs: Node, rhs: Node) -> Node {
        Node {
            kind: NodeKind::Mul(Box::new(lhs), Box::new(rhs)),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }

    pub fn new_div(lhs: Node, rhs: Node) -> Node {
        Node {
            kind: NodeKind::Div(Box::new(lhs), Box::new(rhs)),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }

    pub fn new_eq(lhs: Node, rhs: Node) -> Node {
        Node {
            kind: NodeKind::Eq(Box::new(lhs), Box::new(rhs)),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }

    pub fn new_ne(lhs: Node, rhs: Node) -> Node {
        Node {
            kind: NodeKind::Ne(Box::new(lhs), Box::new(rhs)),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }

    pub fn new_lt(lhs: Node, rhs: Node) -> Node {
        Node {
            kind: NodeKind::Lt(Box::new(lhs), Box::new(rhs)),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }

    pub fn new_le(lhs: Node, rhs: Node) -> Node {
        Node {
            kind: NodeKind::Le(Box::new(lhs), Box::new(rhs)),
            ty: Type::new_int(),
            name: "".to_string(),
        }
    }
}
