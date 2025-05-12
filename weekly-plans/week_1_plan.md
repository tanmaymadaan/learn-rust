### Week 1: Introduction & Setup

**Goal:** Get Rust installed, write your first program, and understand basic concepts like variables, data types, and functions.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

A key resource throughout this plan will be **"The Rust Programming Language" book (TRPL)**, available at [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/). Consider using **Rustlings** ([https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)) for exercises once you're comfortable.

---

**Day 1 (Weekday - 20-30 mins): Understanding Rust & Installation**

- **Objective:** Learn why Rust exists and set up your development environment.
- **Activities:**
  - Read: TRPL - Foreword and Introduction (Chapter 1.1). Focus on Rust's goals: safety, speed, and concurrency. (10-15 mins)
  - Action: Install Rust using `rustup` by following the instructions in TRPL - Chapter 1.1 "Installation". (10-15 mins)
  - Verify installation (`rustc --version`, `cargo --version`).

**Day 2 (Weekday - 20-30 mins): Your First Rust Program**

- **Objective:** Write, compile, and run a "Hello, world!" program.
- **Activities:**
  - Read: TRPL - Chapter 1.2 "Hello, World!". (10 mins)
  - Action:
    - Create a new project: `cargo new hello_world`. (5 mins)
    - Explore the project structure (`Cargo.toml`, `src/main.rs`).
    - Write/modify `src/main.rs` for "Hello, world!".
    - Compile and run: `cargo build`, `cargo run`. (5 mins)
  - Read: TRPL - Chapter 1.3 "Hello, Cargo!". Understand `cargo check`, `cargo build --release`. (5-10 mins)

**Day 3 (Weekday - 20-30 mins): Variables, Mutability & Shadowing**

- **Objective:** Learn how to store and manage data using variables.
- **Activities:**
  - Read: TRPL - Chapter 3.1 "Variables and Mutability". (10-15 mins)
  - Practice:
    - Declare variables (immutable and mutable).
    - Understand constants.
    - Experiment with shadowing.
    - Write small code snippets in your `hello_world` project or a new one (`cargo new variables_practice`). (10-15 mins)

**Day 4 (Weekday - 20-30 mins): Basic Data Types**

- **Objective:** Understand Rust's scalar data types.
- **Activities:**
  - Read: TRPL - Chapter 3.2 "Data Types" (focus on Scalar Types: Integers, Floating-Point Numbers, Booleans, Character Type). (15-20 mins)
  - Practice:
    - Declare variables of each scalar type.
    - Perform simple numeric operations (addition, subtraction).
    - Use boolean values.
    - Work with characters. (5-10 mins)

**Day 5 (Weekday - 20-30 mins): Functions & Comments**

- **Objective:** Learn how to organize code with functions and document it with comments.
- **Activities:**
  - Read: TRPL - Chapter 3.3 "Functions". (10-15 mins)
  - Practice:
    - Define functions with and without parameters.
    - Define functions that return values.
    - Understand the difference between statements and expressions. (10 mins)
  - Read: TRPL - Chapter 3.4 "Comments". (5 mins)
  - Practice: Add comments to your previous code.

**Day 6 (Weekend - 60-90 mins): Review & Compound Types (Part 1)**

- **Objective:** Consolidate understanding of Week 1 topics and get introduced to tuples and arrays.
- **Activities:**
  - Review: Briefly re-read or skim TRPL Chapters 1 and 3.1-3.4. (20-30 mins)
  - Read: TRPL - Chapter 3.2 "Data Types" (Compound Types: Tuples and Arrays). (20-30 mins)
  - Practice:
    - Create and access elements in tuples.
    - Create and access elements in arrays.
    - Write small programs combining variables, functions, and basic data types.
    - Example: A function that takes two numbers, adds them, and prints the result formatted.
    - Example: A function that returns a tuple containing a string and a number. (20-30 mins)
  - Optional: Start Rustlings exercises: `variables`, `functions`.

**Day 7 (Weekend - 60-90 mins): Practice & Preparation for Week 2**

- **Objective:** Solidify learning through more practice and look ahead.
- **Activities:**
  - Practice:
    - Work through more examples from TRPL Chapter 3.
    - Try to complete the first few Rustlings exercises (e.g., `intro`, `variables1-6`, `functions1-5`). (30-45 mins)
  - Problem Solving:
    - Write a small program to convert temperatures (e.g., Fahrenheit to Celsius).
    - Write a function that takes an array of 3 integers and returns their sum. (20-30 mins)
  - Preview: Briefly look at the topics for Week 2 (Control Flow: `if/else`, loops from TRPL Chapter 3.5). (10-15 mins)

---
