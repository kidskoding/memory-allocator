# Memory Allocator - TODO List

- [ ] **Implement Memory Pool Initialization**
  - [ ] Create a function to initialize the memory pool
  with a specified size and chunk size.
  - [ ] Ensure proper alignment and memory allocation using
  `libc`


- [ ] **Implement Memory Allocation Function**
  - [ ] Create a function to allocate memory chunks from
  the pool
  - [ ] Update the free list to reflect allocated chunks

  
- [ ] **Implement Memory Deallocation Function**
  - [ ] Create a function to deallocate memory
  - [ ] Update the free list to reflect deallocated chunks


- [ ] **Implement Safety Checks**
  - [ ] Add safety checks to prevent double-free and
  out-of-bounds access
  - [ ] Ensure proper handling of null pointers


- [ ] **Benchmark Performance**
  - [ ] Create benchmarks to measure the performance of 
  the memory pool allocator
  - [ ] Compare performance with standard dynamic
  memory allocation

  
- [ ] **Handle Multithreading**
  - Implement thread-safe memory allocation 
  and deallocation
  - Use synchronization primitives to protect shared
  resources


- [ ] **Optimize Memory Usage**
  - [ ] Implement strategies to reduce memory
  overhead and fragmentation