### Week 9: Smart Pointers & Concurrency (Basics)

**Goal:** Learn about different smart pointers (`Box`, `Rc`, `RefCell`) for various ownership and mutability scenarios, and get introduced to basic concurrency concepts like threads and message passing.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapter 15 ("Smart Pointers"), Chapter 16.1 ("Using Threads to Run Code Simultaneously"), Chapter 16.2 ("Using Message Passing to Transfer Data Between Threads").
- Rustlings exercises (e.g., `box`, `rc_refcell`, `threads`).

---

**Day 57 (Weekday - 20-30 mins): `Box<T>` for Heap Allocation**

- **Objective:** Understand `Box<T>` for allocating data on the heap and its common use cases (e.g., recursive types, large data).
- **Activities:**
  - Read: TRPL - Chapter 15.1 "Using `Box<T>` to Point to Data on the Heap". (15-20 mins)
  - Practice:
    - Use `Box::new()` to store a simple value on the heap.
    - Define a recursive type (like a cons list from the book) using `Box`. (5-10 mins)

**Day 58 (Weekday - 20-30 mins): The `Deref` Trait**

- **Objective:** Learn how the `Deref` trait allows smart pointers to be treated like regular references.
- **Activities:**
  - Read: TRPL - Chapter 15.2 "Treating Smart Pointers Like Regular References with the `Deref` Trait". (15-20 mins)
  - Practice:
    - Observe how `*` (dereference operator) works with `Box<T>`.
    - (Optional advanced) Try implementing `Deref` for a custom type. (5-10 mins)

**Day 59 (Weekday - 20-30 mins): The `Drop` Trait & Running Code on Cleanup**

- **Objective:** Understand the `Drop` trait for customizing what happens when a value goes out of scope.
- **Activities:**
  - Read: TRPL - Chapter 15.3 "Running Code on Cleanup with the `Drop` Trait". (15-20 mins)
  - Practice: Implement `Drop` for a custom struct and observe when `drop` is called. (5-10 mins)

**Day 60 (Weekday - 20-30 mins): `Rc<T>`, the Reference Counted Smart Pointer**

- **Objective:** Learn about `Rc<T>` for enabling multiple ownership of data (for single-threaded scenarios).
- **Activities:**
  - Read: TRPL - Chapter 15.4 "`Rc<T>`, the Reference Counted Smart Pointer". (15-20 mins)
  - Practice:
    - Use `Rc::new()` and `Rc::clone()` to share ownership of data.
    - Observe reference counting (e.g., with `Rc::strong_count`). (5-10 mins)

**Day 61 (Weekday - 20-30 mins): `RefCell<T>` and the Interior Mutability Pattern**

- **Objective:** Understand `RefCell<T>` for mutating data even when there are immutable references (interior mutability), and its runtime borrow checking.
- **Activities:**
  - Read: TRPL - Chapter 15.5 "`RefCell<T>` and the Interior Mutability Pattern". (15-20 mins)
  - Practice:
    - Use `RefCell` within an `Rc` to have multiple owners who can mutate data.
    - Experiment with `borrow()` and `borrow_mut()`.
    - Intentionally cause a runtime panic by violating borrowing rules with `RefCell`. (5-10 mins)

**Day 62 (Weekend - 60-90 mins): Introduction to Concurrency & Threads**

- **Objective:** Get a conceptual understanding of concurrency vs. parallelism and learn to create basic threads.
- **Activities:**
  - Read: TRPL - Chapter 16.1 "Using Threads to Run Code Simultaneously". (30-40 mins)
  - Practice:
    - Use `thread::spawn` to create new threads.
    - Use `join` handles to wait for threads to finish.
    - Use `move` closures with threads to capture environment data. (30-50 mins)

**Day 63 (Weekend - 60-90 mins): Message Passing with Channels**

- **Objective:** Learn to use channels (`mpsc`) for safe communication between threads.
- **Activities:**
  - Read: TRPL - Chapter 16.2 "Using Message Passing to Transfer Data Between Threads". (30-40 mins)
  - Practice:
    - Create a channel (`mpsc::channel()`).
    - Send and receive data between threads using the transmitter (`tx`) and receiver (`rx`).
    - Experiment with multiple producers. (30-50 mins)
  - Rustlings: Start `box`, `rc_refcell`, and introductory `threads` exercises.
  - Review: Key concepts from TRPL Chapters 15 and 16.1-16.2.

---
