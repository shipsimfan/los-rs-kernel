use super::term::Term;
use crate::parser::{Result, Stream};

pub(crate) struct TermList<'a> {
    stream: Stream<'a>,
}

impl<'a> TermList<'a> {
    pub(crate) fn parse(stream: Stream<'a>) -> Self {
        TermList { stream }
    }
}

impl<'a> Iterator for TermList<'a> {
    type Item = Result<Term<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stream.remaining() != 0 {
            Some(Term::parse(&mut self.stream))
        } else {
            None
        }
    }
}
