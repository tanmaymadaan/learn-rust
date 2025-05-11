### Week 6: Generics, Traits & Lifetimes (Introduction)

**Goal:** Understand how to use generics to reduce code duplication, define shared behavior with traits, and get an initial understanding of lifetimes for ensuring reference validity.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapter 10.1 ("Generic Data Types"), Chapter 10.2 ("Traits: Defining Shared Behavior"), Chapter 10.3 ("Validating References with Lifetimes" - introductory sections).
- Rustlings exercises (e.g., `generics`, `traits`, `lifetimes` - introductory ones).

---

**Day 36 (Weekday - 20-30 mins): Generic Data Types - In Functions & Structs**

- **Objective:** Learn to write generic functions and define generic structs.
- **Activities:**
  - Read: TRPL - Chapter 10.1 "Generic Data Types" (sections: "In Function Definitions", "In Struct Definitions"). (15-20 mins)
  - Practice:
    - Write a generic function (e.g., one that returns the larger of two items, if they support comparison).
    - Define a generic struct (e.g., `Point<T> { x: T, y: T }`). Instantiate it with different types. (5-10 mins)

**Day 37 (Weekday - 20-30 mins): Generic Data Types - In Enums & Methods**

- **Objective:** Understand how generics apply to enums and methods.
- **Activities:**
  - Read: TRPL - Chapter 10.1 "Generic Data Types" (sections: "In Enum Definitions", "In Method Definitions"). (15-20 mins)
  - Practice:
    - Define a generic enum (e.g., `Option<T>` or `Result<T, E>` are good examples to understand, though you won't redefine them).
    - Implement methods on your generic `Point<T>` struct. Consider methods that are generic over `T` and methods that might only work for specific `T` (this leads into trait bounds, which come later). (5-10 mins)

**Day 38 (Weekday - 20-30 mins): Introduction to Traits - Defining & Implementing**

- **Objective:** Learn how to define shared behavior using traits and implement them for types.
- **Activities:**
  - Read: TRPL - Chapter 10.2 "Traits: Defining Shared Behavior" (sections: "Defining a Trait", "Implementing a Trait on a Type"). (15-20 mins)
  - Practice:
    - Define a simple trait (e.g., `Summarizable` with a `summary()` method).
    - Implement this trait for one of your structs or a new simple struct. (5-10 mins)

**Day 39 (Weekday - 20-30 mins): Traits as Parameters & Trait Bounds**

- **Objective:** Understand how to use traits to accept different types that share behavior and how to constrain generics with trait bounds.
- **Activities:**
  - Read: TRPL - Chapter 10.2 "Traits: Defining Shared Behavior" (sections: "Traits as Parameters", "Trait Bound Syntax"). (15-20 mins)
  - Practice:
    - Write a function that takes any type implementing your `Summarizable` trait as an argument.
    - Refactor a generic function/struct to use trait bounds (e.g., your `Point<T>` might require `T` to implement `PartialOrd` for comparison methods). (5-10 mins)
  - Explore deriving common traits like `Debug`, `Clone`, `Copy`.

**Day 40 (Weekday - 20-30 mins): Introduction to Lifetimes - The Problem**

- **Objective:** Understand _why_ lifetimes are necessary in Rust by looking at the problem of dangling references with borrowed values.
- **Activities:**
  - Read: TRPL - Chapter 10.3 "Validating References with Lifetimes" (sections: "Preventing Dangling References with Lifetimes", "The Borrow Checker"). (20-30 mins)
  - Focus on the examples that _don't_ compile without lifetimes, and understand the borrow checker's reasoning. Do not focus heavily on syntax yet, just the concept.

**Day 41 (Weekend - 60-90 mins): Basic Lifetime Syntax & Annotations in Functions**

- **Objective:** Learn the basic syntax for lifetime annotations and how to apply them in simple function signatures where Rust can't infer them.
- **Activities:**
  - Read: TRPL - Chapter 10.3 "Validating References with Lifetimes" (sections: "Lifetime Annotation Syntax", "Lifetime Annotations in Function Signatures"). (30-40 mins)
  - Practice:
    - Take the "longest string" example from the book (or a similar one) and make it compile by adding lifetime annotations.
    - Experiment with functions that take multiple references and return one, annotating lifetimes as needed.
    - Understand the `'a` syntax. (30-50 mins)

**Day 42 (Weekend - 60-90 mins): Review, Practice & Rustlings (Introductory)**

- **Objective:** Consolidate the introductory concepts of generics, traits, and lifetimes.
- **Activities:**
  - Practice Problems:
    - Create a generic function that takes a slice of any type `T` and returns an `Option<T>` (the first element if the slice is not empty, `None` otherwise). `T` will need the `Copy` trait.
    - Define a `Displayable` trait with a method `display(&self) -> String`. Implement it for a few structs. Write a function that takes a `Vec` of `Box<dyn Displayable>` and prints their displays. (This introduces trait objects briefly). (30-45 mins)
  - Rustlings: Start the `generics`, `traits`, and very introductory `lifetimes` exercises. Focus on understanding the compiler messages related to these concepts. (30-45 mins)
  - Review: Skim TRPL Chapter 10.

---
