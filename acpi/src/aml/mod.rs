mod ast;
mod byte_stream;
mod common;
mod data_objects;
mod error;
mod macros;
mod named_objects;
mod namespace_modifier_objects;
mod term_objects;

type Result<T> = core::result::Result<T, Error>;

pub(self) use ast::ASTNode;
pub(self) use byte_stream::ByteStream;
pub(self) use common::*;
pub(self) use data_objects::*;
pub(self) use macros::*;
pub(self) use named_objects::*;
pub(self) use namespace_modifier_objects::*;
pub(self) use term_objects::*;

pub(super) use ast::AML;
pub(super) use error::Error;
