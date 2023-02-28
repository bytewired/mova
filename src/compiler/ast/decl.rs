use super::expr::Expr;
use super::stmt::Stmt;

pub struct VarDecl {
    pub name: String,
    pub expr: Expr,
}

pub struct LetDecl {
    pub name: String,
    pub expr: Expr,
}

pub struct FnDecl {
    pub name: String,
    pub params: Vec<FnParam>,
    pub stmts: Vec<Stmt>,
}

pub struct FnParam {
    // TODO add type, currently by default it's int
    // type_info: TypeInfo
    pub name: String,
    pub is_mutable: bool,
}

pub enum Decl {
    Var(VarDecl),
    Let(LetDecl),
    Fn(FnDecl),
}
