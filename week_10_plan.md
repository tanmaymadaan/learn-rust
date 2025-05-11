### Week 10: Concurrency (Shared State) & Macros

**Goal:** Learn how to share data safely between threads using `Mutex` and `Arc`, and get introduced to declarative macros for metaprogramming.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapter 16.3 ("Shared-State Concurrency"), Chapter 19.6 ("Macros" - focus on Declarative Macros with `macro_rules!`).
- Rustlings exercises (e.g., `mutex`, `arc`, `macros`).

---

**Day 64 (Weekday - 20-30 mins): Introduction to Shared-State Concurrency & Mutexes**

- **Objective:** Understand the challenges of shared state and learn how `Mutex<T>` provides mutual exclusion.
- **Activities:**
  - Read: TRPL - Chapter 16.3 "Shared-State Concurrency" (up to "Atomic Reference Counting with `Arc<T>`"). (15-20 mins)
  - Practice:
    - Try to share and modify a simple value between threads _without_ `Mutex` (observe potential issues or compiler errors if it tries to prevent them).
    - Use `Mutex::new()` and `lock()` to protect shared data. (5-10 mins)

**Day 65 (Weekday - 20-30 mins): Sharing Mutexes Between Threads with `Arc<T>`**

- **Objective:** Learn to use `Arc<T>` (Atomically Reference Counted) to share ownership of a `Mutex` across multiple threads.
- **Activities:**
  - Read: TRPL - Chapter 16.3 (section "Atomic Reference Counting with `Arc<T>`"). (15-20 mins)
  - Practice:
    - Wrap a `Mutex` in an `Arc`.
    - Clone the `Arc` and move it to spawned threads.
    - Access the `Mutex`-protected data from multiple threads. (5-10 mins)

**Day 66 (Weekday - 20-30 mins): `Send` and `Sync` Traits (Conceptual)**

- **Objective:** Understand the `Send` and `Sync` marker traits and how they enable Rust's fearless concurrency (conceptual, no deep dive into implementing them).
- **Activities:**
  - Read: TRPL - Chapter 16.4 "Extensible Concurrency with the `Sync` and `Send` Traits". (20-30 mins)
  - Focus: Understand what these traits signify and why they are important for thread safety.

**Day 67 (Weekday - 20-30 mins): Introduction to Macros - The Problem They Solve**

- **Objective:** Understand why macros are useful and what kind of problems they solve (reducing code duplication, DSLs).
- **Activities:**
  - Read: TRPL - Chapter 19.6 "Macros" (sections "The Difference Between Macros and Functions" and "Declarative Macros with `macro_rules!` for General Metaprogramming" - up to the `vec!` example). (20-30 mins)

**Day 68 (Weekday - 20-30 mins): Declarative Macros with `macro_rules!` (Basics)**

- **Objective:** Learn the basic syntax for defining simple declarative macros.
- **Activities:**
  - Read: TRPL - Chapter 19.6 (continue with the `vec!` example and how `macro_rules!` works). (15-20 mins)
  - Practice:
    - Try to define a very simple macro (e.g., a macro that prints a specific message with a given expression). (5-10 mins)

**Day 69 (Weekend - 60-90 mins): Writing a Simple `macro_rules!` Macro**

- **Objective:** Gain more practice writing a slightly more complex declarative macro.
- **Activities:**
  - Practice:
    - Try to implement a simplified version of the `vec!` macro or a macro that creates a `HashMap` from a list of key-value pairs.
    - Experiment with different metavariable types (`$expr`, `$ident`, `$ty`, etc.). (30-40 mins)
  - Rustlings: Start `macros` exercises. (30-40 mins)
  - Review: Key concepts for `Mutex`, `Arc`.

**Day 70 (Weekend - 60-90 mins): Macro Hygiene & Procedural Macros (Brief Overview)**

- **Objective:** Understand macro hygiene (briefly) and get a conceptual overview of what procedural macros are (without writing them).
- **Activities:**
  - Read: TRPL - Chapter 19.6 (sections on hygiene if available, or search for "Rust macro hygiene" for a basic understanding). (15-20 mins)
  - Read: TRPL - Chapter 19.6 (sections "Procedural Macros for Generating Code from Attributes", "How to Write a Procedural Macro" - conceptual overview only). (20-30 mins)
  - Rustlings: Continue with `macros` exercises. (20-30 mins)
  - Review: Key concepts of declarative macros.

---
