use crate::syntax::exprs::{Expr, IdExpr, PathExpr};

pub trait Stmt {}

pub struct DatDefStmt<'a, E: Expr> {
    id: &'a str,
    t: PathExpr<'a>,
    val: E,
}