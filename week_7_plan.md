### Week 7: Lifetimes In-Depth & Testing

**Goal:** Achieve a deeper understanding of lifetimes (in structs, elision rules, static lifetime) and learn how to write various types of tests in Rust.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapter 10.3 ("Validating References with Lifetimes" - remaining sections), Chapter 11 ("Writing Automated Tests").
- Rustlings exercises (e.g., `lifetimes`, `tests`).

---

**Day 43 (Weekday - 20-30 mins): Lifetime Annotations in Struct Definitions**

- **Objective:** Learn how to use lifetime annotations when structs hold references.
- **Activities:**
  - Read: TRPL - Chapter 10.3 (section "Lifetime Annotations in Struct Definitions"). (15-20 mins)
  - Practice: Define a struct that holds a reference (e.g., `struct Excerpt<'a> { part: &'a str; }`). Instantiate it and understand why the lifetime annotation is needed. (5-10 mins)

**Day 44 (Weekday - 20-30 mins): Lifetime Elision Rules**

- **Objective:** Understand the rules the Rust compiler uses to infer lifetimes in common cases, reducing the need for explicit annotations.
- **Activities:**
  - Read: TRPL - Chapter 10.3 (section "Lifetime Elision"). (15-20 mins)
  - Practice: Review previous functions where lifetimes were explicit. Identify which elision rules apply or would apply. Try removing annotations where elision is possible. (5-10 mins)

**Day 45 (Weekday - 20-30 mins): Static Lifetime & Generic Lifetimes with Traits**

- **Objective:** Learn about the `'static` lifetime and see how lifetimes interact with generic type parameters, trait bounds, and traits.
- **Activities:**
  - Read: TRPL - Chapter 10.3 (sections "Lifetime Annotations in Method Definitions", "The Static Lifetime", "Generic Type Parameters, Trait Bounds, and Lifetimes Together"). (15-20 mins)
  - Practice:
    - Understand string literals and their `'static` lifetime.
    - Briefly look at how lifetimes might appear in more complex generic functions involving traits. (5-10 mins)

**Day 46 (Weekday - 20-30 mins): How to Write Tests - Unit Tests**

- **Objective:** Learn the basics of writing unit tests in Rust.
- **Activities:**
  - Read: TRPL - Chapter 11.1 "How to Write Tests" (up to "Testing Private Functions"). (15-20 mins)
  - Practice:
    - Create a simple function (e.g., `add_two(a: i32) -> i32`).
    - Write a unit test for it using `#[test]` and assertion macros (`assert!`, `assert_eq!`, `assert_ne!`).
    - Run `cargo test`. (5-10 mins)

**Day 47 (Weekday - 20-30 mins): More on Unit Tests - Private Functions & `should_panic`**

- **Objective:** Learn how to test private functions and how to test for panics.
- **Activities:**
  - Read: TRPL - Chapter 11.1 (sections "Testing Private Functions", "Checking for Panics with `should_panic`"). (10-15 mins)
  - Practice:
    - Write a test for a private helper function (if you have one).
    - Write a test using `#[should_panic]` for a function that is expected to panic under certain conditions. (10-15 mins)

**Day 48 (Weekend - 60-90 mins): Controlling How Tests Are Run & Integration Tests**

- **Objective:** Learn to manage test execution and write integration tests.
- **Activities:**
  - Read: TRPL - Chapter 11.2 "Controlling How Tests Are Run". (20-30 mins)
  - Practice: Run tests by name, ignore some tests, run tests in a single thread.
  - Read: TRPL - Chapter 11.3 "Test Organization" (section "Integration Tests"). (20-30 mins)
  - Practice:
    - Create a `tests` directory and an integration test file.
    - Write an integration test for the public API of your library crate (or a simple new one). (20-30 mins)

**Day 49 (Weekend - 60-90 mins): Documentation Tests & Review**

- **Objective:** Learn to write tests within your documentation and review all testing concepts.
- **Activities:**
  - Read: TRPL - Chapter 14.2 "Documentation Comments as Tests" (briefly, as it's more detailed in Ch 14, but the concept is relevant here). Focus on how to write code examples in docs that `cargo test` will run. (15-20 mins)
  - Practice: Add a documentation comment with a runnable example to one of your public functions and test it. (10-15 mins)
  - Rustlings: Work through `lifetimes` and `tests` exercises. (30-40 mins)
  - Review: Skim TRPL Chapters 10.3 and 11. Consolidate understanding of lifetime rules and different test types.

---
