# Error
This crate contains the definition for errors in the kernel.

## Creating an Error
There are three ways to create an error.
 1. Using `Error::new(kind)` which creates an error of the specified kind without a custom message
 2. Using `Error::new_message(kind, message)` which creates an error of the specified kind with a custom message
 3. Using the `From` trait to convert an `ErrorKind`. This creates an error of the specified kind without a custom message

## Error Kinds
The following is a list of error kinds and their explanations
 * **Interrupted** - The operation was interrupted and couldn't complete 
 * **InvalidArgument** - An argument is invalid