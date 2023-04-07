use super::{DataObject, Result};
use crate::{
    interpreter::{term_list, Error, Interpreter},
    namespace::{Namespace, Node},
    parser::TermList,
};
use alloc::{rc::Rc, vec::Vec};
use base::Logger;
use core::cell::RefCell;

pub(crate) fn execute<'a>(
    namespace: &Namespace<'a>,
    method: Rc<RefCell<Node<'a>>>,
    arguments: Vec<DataObject>,
    logger: Logger,
) -> Result<DataObject> {
    // Extract term list and other settings from the method
    let (mut term_list, argument_count, wide_integers, name) = extract_settings(method)?;

    // Verify argument count
    if arguments.len() != argument_count as usize {
        return Err(Error::InvalidArgumentCount(name));
    }

    // Create the interpreter
    let mut interpreter = Interpreter::new(namespace, logger, wide_integers, true);

    // Execute the term list
    Ok(
        match term_list::execute(&mut interpreter, &mut term_list)? {
            Some(data_object) => data_object,
            None => DataObject::Uninitialized,
        },
    )
}

fn extract_settings<'a>(
    method: Rc<RefCell<Node<'a>>>,
) -> Result<(TermList<'a>, u8, bool, [u8; 4])> {
    let method = method.borrow();

    let name = method.name().unwrap();
    let method = match &*method {
        Node::Method(method) => method,
        _ => return Err(Error::InvalidNodeType(name)),
    };

    Ok((
        method.term_list(),
        method.argument_count(),
        method.wide_integers(),
        name,
    ))
}
