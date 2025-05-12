### Week 5: Error Handling & Common Collections

**Goal:** Learn Rust's approaches to error handling (`panic!` and `Result`) and become proficient with common collections like vectors, strings, and hash maps.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapter 9 ("Error Handling"), Chapter 8.1 ("Storing Lists of Values with Vectors"), Chapter 8.2 ("Storing UTF-8 Encoded Text with Strings"), Chapter 8.3 ("Storing Keys with Associated Values in Hash Maps").
- Rustlings exercises (e.g., `error_handling`, `vecs`, `strings`, `hashmap`).

---

**Day 29 (Weekday - 20-30 mins): Unrecoverable Errors with `panic!`**

- **Objective:** Understand how and when Rust uses `panic!` for unrecoverable errors.
- **Activities:**
  - Read: TRPL - Chapter 9.1 "Unrecoverable Errors with `panic!`". (10-15 mins)
  - Practice:
    - Intentionally cause a `panic!` (e.g., by accessing an array out of bounds).
    - Call `panic!("message")` explicitly.
    - Observe the output and backtrace (if enabled). (10-15 mins)

**Day 30 (Weekday - 20-30 mins): Recoverable Errors with `Result`**

- **Objective:** Learn about the `Result<T, E>` enum for handling errors that can be recovered from.
- **Activities:**
  - Read: TRPL - Chapter 9.2 "Recoverable Errors with `Result`". (15-20 mins)
  - Practice:
    - Write a simple function that returns a `Result` (e.g., a division function that returns `Err` for division by zero).
    - Use `match` to handle the `Ok(value)` and `Err(error)` variants. (5-10 mins)

**Day 31 (Weekday - 20-30 mins): Propagating Errors with `?`**

- **Objective:** Understand the `?` operator as a shortcut for propagating errors.
- **Activities:**
  - Read: TRPL - Chapter 9.2 "Recoverable Errors with `Result`" (focus on "A Shortcut for Propagating Errors: the `?` Operator" and "Where The `?` Operator Can Be Used"). (15-20 mins)
  - Practice:
    - Refactor a function that uses `match` on `Result` to use the `?` operator.
    - Create a chain of fallible function calls using `?`. (5-10 mins)

**Day 32 (Weekday - 20-30 mins): Storing Lists with Vectors (`Vec<T>`)**

- **Objective:** Learn to use `Vec<T>` for dynamic, growable lists of items.
- **Activities:**
  - Read: TRPL - Chapter 8.1 "Storing Lists of Values with Vectors". (15-20 mins)
  - Practice:
    - Create, update, and read from vectors.
    - Access elements using indexing and `.get()`.
    - Iterate over vector elements (immutably and mutably).
    - Use vectors to store enum variants with different associated data. (5-10 mins)

**Day 33 (Weekday - 20-30 mins): Storing Text with Strings (`String`)**

- **Objective:** Understand Rust's `String` type for growable, UTF-8 encoded text.
- **Activities:**
  - Read: TRPL - Chapter 8.2 "Storing UTF-8 Encoded Text with Strings". (15-20 mins)
  - Practice:
    - Create and update strings (`String::new()`, `to_string()`, `push_str()`, `+` operator/`format!`).
    - Understand string indexing challenges (bytes vs. characters vs. grapheme clusters).
    - Iterate over characters and bytes in a string. (5-10 mins)

**Day 34 (Weekend - 60-90 mins): Storing Key-Value Pairs with Hash Maps (`HashMap<K, V>`)**

- **Objective:** Learn to use `HashMap<K, V>` for associative data storage.
- **Activities:**
  - Read: TRPL - Chapter 8.3 "Storing Keys with Associated Values in Hash Maps". (20-30 mins)
  - Practice:
    - Create, access, and update hash maps.
    - Iterate over key-value pairs in a hash map.
    - Understand ownership with hash maps (keys and values).
    - Explore common hash map operations (e.g., `entry` API for conditional updates). (30-40 mins)
  - Optional: Start Rustlings `vecs`, `strings`, `hashmap` exercises.

**Day 35 (Weekend - 60-90 mins): Error Handling & Collections Practice**

- **Objective:** Consolidate learning by combining error handling and collections in practical scenarios.
- **Activities:**
  - Practice Problems:
    - Write a function that reads numbers from a string (e.g., "1,2,error,4"), parses them into `i32`, collects them into a `Vec<i32>`, and returns `Result<Vec<i32>, ParseIntError>`. Use `?`.
    - Create a program that counts word frequencies in a given text using a `HashMap`. (30-45 mins)
  - Rustlings: Work through exercises in `error_handling/`, `vecs/`, `strings/`, `hashmap/`. (30-45 mins)
  - Review: Skim TRPL Chapters 8 and 9.

---
