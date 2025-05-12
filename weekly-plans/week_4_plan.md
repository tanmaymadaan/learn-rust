### Week 4: Structs & Enums

**Goal:** Learn to define custom data types using structs and enums, understand how to define behavior with methods, and use the powerful `match` control flow construct and the `Option` enum.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapter 5 ("Using Structs to Structure Related Data"), Chapter 6.1 ("Defining an Enum"), Chapter 6.2 ("The `match` Control Flow Construct").
- Rustlings exercises (e.g., `structs`, `enums`, `option`, `match_statement_guards`).

---

**Day 22 (Weekday - 20-30 mins): Defining and Instantiating Structs**

- **Objective:** Learn the basics of creating custom types with structs.
- **Activities:**
  - Read: TRPL - Chapter 5.1 "Defining and Instantiating Structs". (10-15 mins)
  - Practice:
    - Define a few simple structs (e.g., `User`, `Color`, `Point`).
    - Create instances of your structs.
    - Access and modify struct fields (if mutable).
    - Use field init shorthand when variables have the same name as fields.
    - Understand struct update syntax. (10-15 mins)

**Day 23 (Weekday - 20-30 mins): Structs Example & Tuple Structs**

- **Objective:** See structs in a practical example and learn about tuple structs.
- **Activities:**
  - Read: TRPL - Chapter 5.2 "An Example Program Using Structs" (the rectangle example). Focus on how structs help organize the data and logic. (10-15 mins)
  - Revisit: TRPL - Chapter 5.1 "Tuple Structs without Named Fields".
  - Practice:
    - Implement a small part of the rectangle example or a similar geometric shape struct.
    - Define and use a tuple struct (e.g., for `Color(u8, u8, u8)` or `Point(i32, i32)`). (10-15 mins)

**Day 24 (Weekday - 20-30 mins): Method Syntax with `impl`**

- **Objective:** Learn how to define methods associated with structs.
- **Activities:**
  - Read: TRPL - Chapter 5.3 "Method Syntax". Pay attention to `&self`, `&mut self`, and `self`. (15-20 mins)
  - Practice:
    - Add methods to your previously defined structs (e.g., a `area` method for the rectangle).
    - Create associated functions (like constructors, e.g., `User::new(...)`). (5-10 mins)

**Day 25 (Weekday - 20-30 mins): Defining Enums**

- **Objective:** Understand how to define enumerations for types that can be one of a few variants.
- **Activities:**
  - Read: TRPL - Chapter 6.1 "Defining an Enum". (15-20 mins)
  - Practice:
    - Define an enum like `IpAddrKind (V4, V6)`.
    - Define an enum that can hold data, e.g., `IpAddr { kind: IpAddrKind, address: String, }` or `Message { Quit, Move { x: i32, y: i32 }, Write(String), ChangeColor(i32, i32, i32), }`.
    - Create instances of your enum variants. (5-10 mins)

**Day 26 (Weekday - 20-30 mins): The `match` Control Flow Construct**

- **Objective:** Learn to use `match` for exhaustive pattern matching on enums and other values.
- **Activities:**
  - Read: TRPL - Chapter 6.2 "The `match` Control Flow Construct". (15-20 mins)
  - Practice:
    - Write functions that take your enums as arguments and use `match` to perform different actions based on the variant.
    - Ensure all patterns are covered (the compiler will help!).
    - Use the `_` placeholder for catch-all patterns. (5-10 mins)

**Day 27 (Weekend - 60-90 mins): The `Option` Enum & `match` with `Option<T>`**

- **Objective:** Understand the `Option` enum for handling values that might be absent, and how `match` works with it.
- **Activities:**
  - Read: TRPL - Chapter 6.1 (re-read "The `Option` Enum and Its Advantages Over Null Values"). (15-20 mins)
  - Read: TRPL - Chapter 6.2 (re-read "Patterns that Bind to Values" and "Matching with `Option<T>`"). (15-20 mins)
  - Practice:
    - Write functions that return `Option<T>` (e.g., a function that finds an item and might not find it).
    - Use `match` to handle `Some(value)` and `None` cases from your functions.
    - Explore `if let` as a concise alternative to `match` for simple cases (TRPL Chapter 6.3 - though the main focus is `match` here). (30-50 mins)

**Day 28 (Weekend - 60-90 mins): Review, Practice & Rustlings**

- **Objective:** Consolidate knowledge of structs and enums through more complex examples and exercises.
- **Activities:**
  - Practice Problems:
    - Define a struct for an item in an e-commerce cart (e.g., `product_name: String, quantity: u32, unit_price: f32`). Implement methods like `calculate_total_price`.
    - Define an enum representing different states of a traffic light (`Red`, `Yellow`, `Green`). Write a function that takes a state and returns the duration it should stay in that state. Use `match`. (30-45 mins)
  - Rustlings: Work through exercises in `structs/`, `enums/`, `option/`, and `match_statement_guards/`. (30-45 mins)
  - Review: Skim TRPL Chapters 5 and relevant parts of 6 to reinforce concepts.

---
