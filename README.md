# 90-Day Rust Study Plan

This plan is designed to help you learn Rust over approximately 90 days, dedicating 20-30 minutes on weekdays and 60-90 minutes on weekends.

## Week-on-Week High-Level Plan

**Week 1: Introduction & Setup**

- Understanding Rust: Core philosophies (Safety, Speed, Concurrency).
- Installation: Setting up `rustup`, `rustc`, and `cargo`.
- Your First Program: "Hello, World!" and basic `cargo` commands (`cargo new`, `cargo build`, `cargo run`).
- Fundamental Concepts: Variables, mutability, shadowing.
- Basic Data Types: Integers, floating-point numbers, booleans, characters.
- Functions: Defining and calling functions, parameters, return values.
- Comments and `println!` macro for output.

**Week 2: Control Flow & Basic Data Structures**

- Control Flow: `if/else` expressions, `else if`.
- Loops: `loop`, `while`, and `for` loops.
- Basic Data Structures:
  - Tuples: Fixed-size ordered lists of elements of different types.
  - Arrays: Fixed-size lists of elements of the same type.
- Introduction to Ownership: Conceptual overview (what it is, why it's important).
- Practical Project: The guessing game from Chapter 2 of TRPL, incorporating user input, random number generation, match expressions, and loops.

**Week 3: Ownership & Borrowing**

- Ownership Deep Dive:
  - The Stack and the Heap.
  - Ownership rules.
  - Move semantics.
  - Clone trait for duplicating data.
- References & Borrowing:
  - Immutable references (`&T`).
  - Mutable references (`&mut T`).
  - Rules of borrowing (one mutable reference or any number of immutable references).
- Slices: Referencing a contiguous sequence of elements in a collection.
- Understanding the Borrow Checker: How Rust enforces memory safety at compile time.

**Week 4: Structs & Enums**

- Structs: Defining custom data types.
  - Creating instances of structs.
  - Struct update syntax.
  - Tuple structs.
- Methods: Defining behavior associated with structs using `impl` blocks.
  - Associated functions (e.g., constructors like `String::from`).
- Enums (Enumerations): Defining a type with a few definite variants.
  - `match` expressions for powerful control flow with enums.
  - The `Option` enum for handling possibly absent values.
  - Enums with data.

**Week 5: Error Handling & Common Collections**

- Error Handling:
  - Unrecoverable errors with `panic!`.
  - Recoverable errors with the `Result<T, E>` enum.
  - Propagating errors with the `?` operator.
- Common Collections:
  - Vectors (`Vec<T>`): Growable arrays.
  - Strings (`String`): UTF-8 encoded, growable text.
  - Hash Maps (`HashMap<K, V>`): Storing key-value pairs.
- Iterating over collections.

**Week 6: Generics, Traits & Lifetimes (Introduction)**

- Generics: Writing code that can operate on different concrete types.
  - In function definitions.
  - In struct definitions.
  - In enum definitions.
- Traits: Defining shared behavior (similar to interfaces in other languages).
  - Defining traits.
  - Implementing traits on types.
  - Using trait bounds to constrain generic types.
  - Deriving common traits (`Debug`, `Clone`, `Copy`, etc.).
- Lifetimes (Introduction): Understanding why lifetimes are needed to ensure references are always valid. Basic syntax.

**Week 7: Lifetimes In-Depth & Testing**

- Lifetimes:
  - Lifetime annotations in function signatures.
  - Lifetime annotations in struct definitions.
  - Lifetime elision rules.
  - Static lifetime.
- Testing in Rust:
  - Writing unit tests with the `#[test]` attribute.
  - Running tests with `cargo test`.
  - `assert!`, `assert_eq!`, `assert_ne!` macros.
  - Controlling how tests are run (e.g., ignoring tests, running specific tests).
  - Test organization (tests module).
  - Integration tests.
  - Documentation tests.

**Week 8: Modules, Crates & Cargo In-depth**

- Managing Growing Projects with Packages, Crates, and Modules:
  - Packages and Crates: What they are (binary vs. library).
  - Modules: Organizing code within a crate.
  - Paths for referring to items in the module tree (`::` operator).
  - The `use` keyword for bringing paths into scope.
  - Controlling visibility with `pub`.
- Cargo In-depth:
  - Understanding `Cargo.toml` (manifest file).
  - Dependencies: Adding external crates from crates.io.
  - Workspaces for managing multiple related packages.
  - Profiles (dev, release).

**Week 9: Smart Pointers & Concurrency (Basics)**

- Smart Pointers: Pointers with additional metadata and capabilities.
  - `Box<T>`: For allocating values on the heap.
  - `Rc<T>` (Reference Counting): Enabling multiple ownership of data on the heap.
  - `RefCell<T>` and the interior mutability pattern.
- Concurrency (Introduction):
  - "Fearless Concurrency" in Rust.
  - Creating new threads with `thread::spawn`.
  - Using `join` handles to wait for threads to complete.
- Message Passing: Transferring data between threads using channels (`std::sync::mpsc`).

**Week 10: Concurrency (Shared State) & Macros**

- Shared-State Concurrency:
  - Mutexes (`std::sync::Mutex<T>`) for allowing only one thread to access data at a time.
  - `Arc<T>` (Atomically Reference Counted): For sharing ownership across threads safely.
- Macros: Writing code that writes other code (metaprogramming).
  - Declarative macros with `macro_rules!` for general metaprogramming.
  - Procedural macros for more complex code generation (custom `#[derive]`, attribute-like, function-like).
  - When and how to use basic macros.

**Week 11: Project Focus & Advanced Topics Introduction**

- Project Work: Start building a small, practical Rust project.
  - Ideas: Command-line tool (e.g., a simple grep clone, task manager), a basic text adventure game, a simple file utility.
  - Focus on applying concepts learned so far.
- Introduction to an Advanced Topic (choose one or explore briefly):
  - Asynchronous Programming: `async/await` for non-blocking operations (e.g., with Tokio or async-std).
  - Foreign Function Interface (FFI): Calling C code from Rust and Rust code from C.
  - Unsafe Rust: Opting out of some of Rust's guarantees (and why you might need to).

**Week 12: Project Completion & Review**

- Continue Project Work: Aim to complete a functional version of your chosen project.
  - Refactor and improve code.
  - Add tests.
  - Document your project.
- Comprehensive Review: Go over the key concepts from the past 11 weeks.
  - Revisit challenging topics.
  - Read documentation and articles on areas you want to solidify.
- Plan Next Steps:
  - Identify areas of Rust you want to explore further (e.g., web development with Actix/Axum, game development with Bevy, embedded systems, systems programming).
  - Find more complex projects or contribute to open-source.

---

This is the high-level weekly structure. Detailed day-by-day plans for each week will be provided in separate files (e.g., `week_1_plan.md`, `week_2_plan.md`, etc.).

For Week 1, see: [Link to Week 1 Detailed Plan](./week_1_plan.md)
For Week 2, see: [Link to Week 2 Detailed Plan](./week_2_plan.md)
For Week 3, see: [Link to Week 3 Detailed Plan](./week_3_plan.md)
For Week 4, see: [Link to Week 4 Detailed Plan](./week_4_plan.md)
For Week 5, see: [Link to Week 5 Detailed Plan](./week_5_plan.md)
For Week 6, see: [Link to Week 6 Detailed Plan](./week_6_plan.md)
For Week 7, see: [Link to Week 7 Detailed Plan](./week_7_plan.md)
For Week 8, see: [Link to Week 8 Detailed Plan](./week_8_plan.md)
For Week 9, see: [Link to Week 9 Detailed Plan](./week_9_plan.md)
For Week 10, see: [Link to Week 10 Detailed Plan](./week_10_plan.md)
For Week 11, see: [Link to Week 11 Detailed Plan](./week_11_plan.md)
For Week 12, see: [Link to Week 12 Detailed Plan](./week_12_plan.md)
