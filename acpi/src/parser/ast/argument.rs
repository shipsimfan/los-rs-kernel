use super::{DataObject, Expression};
use crate::parser::{ArgObj, Context, Error, LocalObj, Result, Stream};

pub(crate) enum Argument<'a> {
    Arg(ArgObj),
    DataObject(DataObject<'a>),
    Expression(Expression<'a>),
    Local(LocalObj),
}

impl<'a> Argument<'a> {
    pub(in crate::parser::ast) fn parse(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Self> {
        match Argument::parse_opt(stream, context)? {
            Some(argument) => Ok(argument),
            None => Err(Error::unexpected_byte(
                stream.next().unwrap(),
                stream.offset() - 1,
                "Argument",
            )),
        }
    }

    pub(super) fn parse_opt(
        stream: &mut Stream<'a>,
        context: &mut Context,
    ) -> Result<Option<Self>> {
        if let Some(data_object) = DataObject::parse_opt(stream, context)? {
            return Ok(Some(Argument::DataObject(data_object)));
        }

        if let Some(arg_obj) = ArgObj::parse(stream)? {
            return Ok(Some(Argument::Arg(arg_obj)));
        }

        if let Some(local_obj) = LocalObj::parse(stream)? {
            return Ok(Some(Argument::Local(local_obj)));
        }

        Ok(Expression::parse_opt(stream, context)?
            .map(|expression| Argument::Expression(expression)))
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
