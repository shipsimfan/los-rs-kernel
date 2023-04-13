use crate::{
    parser::{ast::Argument, name_string, Context, Result, Stream},
    Path,
};
use alloc::vec::Vec;
use base::{log_debug, Logger};

pub(crate) struct MethodInvocation<'a> {
    path: Path,
    arg_list: Vec<Argument<'a>>,
}

impl<'a> MethodInvocation<'a> {
    pub(super) fn parse(stream: &mut Stream<'a>, context: &mut Context) -> Result<Self> {
        let path = name_string::parse(stream, "Method Invocation")?;

        let argument_count = context.get_method_argument_count(&path);

        let mut arg_list = Vec::with_capacity(argument_count);

        let logger: Logger = "MethodInvoke".into();
        log_debug!(logger, "{} Argument count: {}", path, argument_count);

        for _ in 0..argument_count {
            if stream.remaining() == 0 {
                break;
            }

            arg_list.push(Argument::parse(stream, context)?);
        }

        Ok(MethodInvocation { path, arg_list })
    }
}

impl<'a> core::fmt::Display for MethodInvocation<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} (", self.path)?;

        for i in 0..self.arg_list.len() {
            write!(f, "{}", self.arg_list[i])?;

            if i < self.arg_list.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, ")")
    }
}
