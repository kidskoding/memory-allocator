extern crate libc;
use std::ptr::NonNull;

struct MemoryPool {
    block: NonNull<u8>,
    chunk_size: usize,
    free_list: Option<NonNull<Node>>
}

struct Node {
    next: Option<NonNull<Node>>
}