# Critical
Critical refers to whether or not a thread can be interrupted at a given time.

## Local Critical
Local critical controls the interruptability on the current core. It is tracked using an unsigned integer, only allowing interruption when the count is zero. Three functions exist to control the current state of local critical:
  - enter_local() - Increments the local critical count and disables interrupts.
  - leave_local() - Decrements the local critical count and enables interrupts if it is zero.
  - leave_local_without_sti() - Decrements the local critical count but doesn't enable interrupts.

## Critical Locks
Critical locks are spinlocks that garuntee being in local critical while the lock is held. Critical locks have three functions for interacting with them:
 1. new(data) - Creates a new critical lock guarding *data*.
 2. lock() - Enters local critical and locks the spinlock, returns a guard that can be dereferences to access the data.
 3. is_locked() - Returns a boolean indicating the current state of the lock.

Critical locks will automatically be unlocked when the guard is dropped.