use super::{DataObject, Expression};
use crate::parser::{ArgObj, Context, LocalObj, Result, Stream};

pub(crate) enum Argument<'a> {
    Arg(ArgObj),
    DataObject(DataObject<'a>),
    Expression(Expression<'a>),
    Local(LocalObj),
}

impl<'a> Argument<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        if let Some(expression) = Expression::parse(stream, context)? {
            return Ok(Argument::Expression(expression));
        }

        if let Some(arg_obj) = ArgObj::parse(stream)? {
            return Ok(Argument::Arg(arg_obj));
        }

        if let Some(local_obj) = LocalObj::parse(stream)? {
            return Ok(Argument::Local(local_obj));
        }

        DataObject::parse(stream, context).map(|data_object| Argument::DataObject(data_object))
    }
}

impl<'a> core::fmt::Display for Argument<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Argument::Arg(arg_obj) => arg_obj.fmt(f),
            Argument::DataObject(data_object) => data_object.fmt(f),
            Argument::Expression(expression) => expression.fmt(f),
            Argument::Local(local_obj) => local_obj.fmt(f),
        }
    }
}
