pub trait Expr {}

pub struct PathExpr<'a> {
    ids: Vec<&'a str>,
}

impl<'a> Expr for PathExpr<'a> {}

pub enum Binop {
    Add,
    Sub,
    Multi,
    Div,
    Mod,
}

pub struct BinopExpr<E: Expr> {
    kind: Binop,
    expr: [E; 2],
}

impl Expr for BinopExpr<E> {}

pub enum LitExpr<'a> {
    Bool(bool),
    Int(&'a str),
    Float(&'a str),
    Char(char),
    String(&'a str),
}

impl<'a> Expr for LitExpr<'a> {}