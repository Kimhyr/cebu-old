use std::fmt::Binary;

use super::token::{Token, BinaryToken, LiteralToken};

pub trait Expr {}

pub struct IdExpr {
    id: String
}

impl IdExpr {
    pub fn new(id: String) -> Self {
        Self {
            id
        }
    }
}

impl Expr for IdExpr {
}

pub struct LiteralExpr {
    literal: LiteralToken
}

impl LiteralExpr {
    pub fn new(literal: LiteralToken) -> Self {
        Self {
            literal
        }
    }
}

impl Expr for LiteralExpr {
}

pub struct PathExpr {
    ids: Vec<IdExpr>
}

impl PathExpr {
    pub fn new(ids: Vec<IdExpr>) -> Self{
        Self {
            ids
        }
    }
}

impl Expr for PathExpr {
}

pub struct BinaryExpr<A: Expr, B: Expr> {
    op: BinaryToken,
    exprs: (A, B)
}

impl<A: Expr, B: Expr> Expr for BinaryExpr<A, B> {
}