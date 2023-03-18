use core::ptr::NonNull;

pub(super) struct Node {
    next: Option<NonNull<Node>>,
}

impl Node {
    pub(super) fn new(next: Option<NonNull<Node>>) -> Self {
        Node { next }
    }

    pub(super) fn take_next(&mut self) -> Option<NonNull<Node>> {
        self.next.take()
    }

    pub(super) fn set_next(&mut self, next: Option<NonNull<Node>>) {
        self.next = next;
    }
}
