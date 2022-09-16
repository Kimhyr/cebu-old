use super::exprs::{IdExpr, PathExpr, Expr};

pub trait Stmt {
}

pub struct DatStmt<E: Expr> {
    id: IdExpr,
    path: PathExpr,
    value: E,
}

impl<E: Expr> Stmt for DatStmt<E> {
}