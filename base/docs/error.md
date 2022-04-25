# Error
Provides a standard definition for errors across the kernel.

## Module Number
The module number has to be unique to a given module so that errors do not overlap when passed to userspace. The module number can be any value from 0 to 2^31 - 1. Negative module numbers are invalid and will cause unexpected behaviour. Module numbers below 0-127 are reserved for the kernel. Drivers may use any other number or use errors provided by the kernel.

## Error Number
This number specifies the specific error that has occurred and is combined with the module number when passed to userspace. The error number can also be any number from 0 to 2^32 - 1.