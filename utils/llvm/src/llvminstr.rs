use crate::{label::Label, llvmop::*, llvmvar::VarType, temp::Temp};
use std::fmt::Display;

pub struct GlobalVar {}

pub trait LlvmInstr: Display {
	fn get_read(&self) -> Vec<Temp> {
		Vec::new()
	}
	fn get_write(&self) -> Vec<Temp> {
		Vec::new()
	}
	fn type_valid(&self) -> bool {
		true
	}
	fn is_label(&self) -> bool {
		false
	}
	fn is_seq(&self) -> bool {
		true
	}
	fn is_ret(&self) -> bool {
		false
	}
}

pub struct ArithInstr {
	pub target: Temp,
	pub op: ArithOp,
	pub var_type: VarType,
	pub lhs: Value,
	pub rhs: Value,
}

pub struct LabelInstr {
	pub label: Label,
}

pub struct CompInstr {
	pub kind: CompKind,
	pub target: Temp,
	pub op: CompOp,
	pub var_type: VarType,
	pub lhs: Value,
	pub rhs: Value,
}

pub struct ConvertInstr {
	pub target: Temp,
	pub op: ConvertOp,
	pub var_type: VarType,
	pub lhs: Value,
	pub rhs: Value,
}

pub struct JumpInstr {
	pub target: Label,
}

pub struct JumpCondInstr {
	pub var_type: VarType,
	pub cond: Value,
	pub target_true: Label,
	pub target_false: Label,
}

pub struct PhiInstr {
	pub target: Temp,
	pub var_type: VarType,
	pub source: Vec<(Value, Label)>,
}

pub struct RetInstr {
	pub value: Value,
}
