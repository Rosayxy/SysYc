use std::fmt::Debug;
use sysyc_derive::{has_attrs, AstNode};
use utils::{Attr, Attrs, SysycError};

use crate::{visitor::Visitor, BinaryOp, FuncType, UnaryOp, VarType};

pub trait AstNode: Debug + Attrs {
	fn accept(&mut self, visitor: &mut dyn Visitor) -> Result<(), SysycError>;
}

pub type Node = Box<dyn AstNode>;
pub type NodeList = Vec<Node>;

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct Program {
	pub comp_units: NodeList,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct VarDef {
	pub ident: String,
	pub dim_list: Option<NodeList>,
	pub init: Option<Node>,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct VarDecl {
	pub is_const: bool,
	pub type_t: VarType,
	pub defs: NodeList,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct InitValList {
	pub val_list: NodeList,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct LiteralInt {
	pub value: i32,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct LiteralFloat {
	pub value: f32,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct Lval {
	pub ident: String,
	pub dim_list: Option<NodeList>,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct BinaryExpr {
	pub lhs: Node,
	pub op: BinaryOp,
	pub rhs: Node,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct UnaryExpr {
	pub op: UnaryOp,
	pub rhs: Node,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct FuncCall {
	pub ident: String,
	pub params: NodeList,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct FuncDecl {
	pub func_type: FuncType,
	pub ident: String,
	pub formal_params: NodeList,
	pub block: Node,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct FormalParam {
	pub type_t: VarType,
	pub ident: String,
	pub dim_list: Option<NodeList>,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct Block {
	pub stmts: NodeList,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct If {
	pub cond: Node,
	pub body: Node,
	pub then: Option<Node>,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct While {
	pub cond: Node,
	pub body: Node,
}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct Break {}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct Continue {}

#[derive(Debug, AstNode)]
#[has_attrs]
pub struct Return {
	pub value: Option<Node>,
}
