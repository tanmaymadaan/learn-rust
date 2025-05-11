### Week 3: Ownership & Borrowing Deep Dive

**Goal:** Gain a solid understanding of references, borrowing rules, and slices, and how they work with the ownership system to ensure memory safety.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapters 4.2, 4.3. Review concepts from 4.1 as needed.
- Rustlings exercises (especially `borrowing`, `slices`).

---

**Day 15 (Weekday - 20-30 mins): Revisiting Ownership & Introducing References**

- **Objective:** Briefly review core ownership rules and introduce the concept of references.
- **Activities:**
  - Quick Review: Skim TRPL Chapter 4.1 "What is Ownership?". Re-affirm the three ownership rules. (5-10 mins)
  - Read: TRPL - Chapter 4.2 "References and Borrowing" (up to "Mutable References"). Focus on what references are and how they differ from taking ownership. (15-20 mins)
  - Practice: Write a function that takes a `String` by ownership and another that takes a reference (`&String`). Observe the difference in how you call them and what happens to the original `String`.

**Day 16 (Weekday - 20-30 mins): Immutable References**

- **Objective:** Understand how immutable references work and the rules governing them.
- **Activities:**
  - Read: TRPL - Chapter 4.2 "References and Borrowing" (specifically on immutable references and their scope). (10-15 mins)
  - Practice:
    - Create multiple immutable references to the same piece of data.
    - Pass immutable references to functions.
    - Observe what happens if you try to modify data through an immutable reference (compiler error). (10-15 mins)

**Day 17 (Weekday - 20-30 mins): Mutable References**

- **Objective:** Learn how to use mutable references to change borrowed data and understand their stricter rules.
- **Activities:**
  - Read: TRPL - Chapter 4.2 "References and Borrowing" (section on "Mutable References"). Pay close attention to the "one mutable reference" rule. (10-15 mins)
  - Practice:
    - Create a mutable reference and modify data through it.
    - Experiment with the borrow checker:
      - Try creating two mutable references to the same data in the same scope.
      - Try creating a mutable reference while an immutable one exists. (10-15 mins)
    - Understand how scopes can allow multiple mutable references (but not simultaneous ones).

**Day 18 (Weekday - 20-30 mins): Dangling References**

- **Objective:** Understand what dangling references are and how Rust's borrow checker prevents them.
- **Activities:**
  - Read: TRPL - Chapter 4.2 "References and Borrowing" (section on "Dangling References"). (10-15 mins)
  - Conceptual: Think about why dangling references are a problem in languages like C/C++.
  - Experiment: Try to write code that _would_ create a dangling reference (e.g., returning a reference to a variable that goes out of scope). Observe the compiler error and understand its message. (10-15 mins)

**Day 19 (Weekday - 20-30 mins): Introduction to Slices - String Slices**

- **Objective:** Learn about string slices as a way to reference parts of a String.
- **Activities:**
  - Read: TRPL - Chapter 4.3 "The Slice Type" (up to "Other Slices"). Focus on string slices. (15-20 mins)
  - Practice:
    - Write a function that takes a string slice (`&str`) as a parameter.
    - Create string slices from a `String`.
    - Pass string literals (which are slices) to your function. (5-10 mins)

**Day 20 (Weekend - 60-90 mins): Other Slices & Borrow Checker Wisdom**

- **Objective:** Explore slices for other collection types and reinforce understanding of the borrow checker's role.
- **Activities:**
  - Read: TRPL - Chapter 4.3 "The Slice Type" (section on "Other Slices" and the summary). (20-30 mins)
  - Practice:
    - Create slices from arrays.
    - Write a function that takes an array slice (e.g., `&[i32]`).
    - Experiment with slices and ownership/borrowing rules. (20-30 mins)
  - Review: The core rules of references from TRPL 4.2. How do these rules prevent data races at compile time? (10-15 mins)
  - Optional: Start Rustlings `borrowing` and `slices` exercises.

**Day 21 (Weekend - 60-90 mins): Practice, Rustlings & Conceptual Review**

- **Objective:** Solidify understanding of ownership, borrowing, and slices through extensive practice.
- **Activities:**
  - Practice Problems:
    - Write a function `first_word(s: &String) -> &str` that returns a slice of the first word in a `String`.
    - Write a function that takes a slice of integers and returns the largest number in the slice. Handle empty slices gracefully (perhaps by returning `Option<&i32>`). (30-45 mins)
  - Rustlings: Work through `borrowing` and `slices` exercises in Rustlings. Focus on understanding the compiler errors and fixing them. (30-45 mins)
  - Conceptual Review: Explain to yourself (or write down) the relationship between ownership, borrowing, and slices. Why are they designed this way? What problems do they solve?

---
