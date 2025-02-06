# Memory Allocator - TODO List

- [x] **Implement Memory Pool Initialization**
  - [x] Create a function to initialize the memory pool
  with a specified size and chunk size.
  - [x] Ensure proper alignment and memory allocation using
  `libc`


- [x] **Implement Memory Allocation Function**
  - [x] Create a function to allocate memory chunks from
  the pool
  - [x] Update the free list to reflect allocated chunks

  
- [x] **Implement Memory Deallocation Function**
  - [x] Create a function to deallocate memory
  - [x] Update the free list to reflect deallocated chunks


- [x] **Implement Safety Checks**
  - [x] Add safety checks to prevent double-free and
  out-of-bounds access
  - [x] Ensure proper handling of null pointers


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