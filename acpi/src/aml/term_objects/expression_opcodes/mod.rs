mod deref_of;
mod index;
mod ref_of;
mod reference_type_opcode;

pub(self) use deref_of::DerefOf;
pub(self) use index::Index;
pub(self) use ref_of::RefOf;

pub(in crate::aml) use reference_type_opcode::ReferenceTypeOpcode;
