### Week 2: Control Flow & Introduction to Ownership

**Goal:** Master Rust's control flow mechanisms (conditionals and loops) and get a foundational understanding of Rust's ownership system.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapters 2, 3.5, 4.1.
- Rustlings exercises (optional, but highly recommended).

---

**Day 8 (Weekday - 20-30 mins): Conditional Execution with `if/else`**

- **Objective:** Learn to control program flow based on conditions.
- **Activities:**
  - Read: TRPL - Chapter 3.5 "Control Flow" (focus on `if` Expressions, `else if`). (10-15 mins)
  - Practice:
    - Write small programs using `if`, `else if`, and `else`.
    - Experiment with using `if` in a `let` statement.
    - Example: A program that checks if a number is positive, negative, or zero. (10-15 mins)

**Day 9 (Weekday - 20-30 mins): Repetitive Execution with `loop` and `while`**

- **Objective:** Understand how to create indefinite loops and condition-based loops.
- **Activities:**
  - Read: TRPL - Chapter 3.5 "Control Flow" (focus on `loop`, Returning Values from Loops, `while` loops). (10-15 mins)
  - Practice:
    - Use `loop` with `break` to exit.
    - Use `loop` to return a value.
    - Implement a countdown using a `while` loop. (10-15 mins)

**Day 10 (Weekday - 20-30 mins): Iterating with `for` Loops**

- **Objective:** Learn to iterate over collections and ranges.
- **Activities:**
  - Read: TRPL - Chapter 3.5 "Control Flow" (focus on Looping Through a Collection with `for`, using `Range`). (10-15 mins)
  - Practice:
    - Use `for` to iterate over an array's elements.
    - Use `for` with a `Range` to execute code a specific number of times (e.g., `1..5`).
    - Use `.rev()` to reverse a range. (10-15 mins)

**Day 11 (Weekday - 20-30 mins): Control Flow Practice & Review**

- **Objective:** Consolidate understanding of all control flow structures.
- **Activities:**
  - Practice:
    - Problem 1: Write a function that takes an integer and prints "Fizz" if it's divisible by 3, "Buzz" if divisible by 5, and "FizzBuzz" if divisible by both. Print the number otherwise. Loop from 1 to 30.
    - Problem 2 (optional, can extend to weekend): Try to generate the first N Fibonacci numbers. (15-20 mins)
  - Review: Quickly skim TRPL Chapter 3.5. (5-10 mins)

**Day 12 (Weekday - 20-30 mins): What is Ownership? (Conceptual)**

- **Objective:** Get a high-level understanding of Rust's ownership system, the stack, and the heap.
- **Activities:**
  - Read: TRPL - Chapter 4.1 "What is Ownership?" (Sections: What is Ownership?, The Stack and the Heap, Ownership Rules, Variable Scope). (20-30 mins)
  - Focus on understanding _why_ ownership is important and the three main rules. Don't worry about mastering it yet, just get the concepts.

**Day 13 (Weekend - 60-90 mins): Ownership in Action - `String`, Memory, Move Semantics**

- **Objective:** See how ownership rules apply to heap-allocated data like `String` and understand 'move' semantics.
- **Activities:**
  - Read: TRPL - Chapter 4.1 "What is Ownership?" (Sections: The `String` Type, Memory and Allocation, Ways Variables and Data Interact: Move). (30-40 mins)
  - Practice:
    - Create `String` variables.
    - Assign one `String` variable to another and observe the move (try using the original variable).
    - Pass a `String` to a function and see how ownership is moved into the function.
    - Note down any compiler errors and understand why they happen. (30-40 mins)
  - Optional: Start Rustlings exercises: `if1`, `if2`, `quiz1` (if not done), `move_semantics1`, `move_semantics2`.

**Day 14 (Weekend - 60-90 mins): Duplicating Data (`Clone`) & Stack-Only Data (`Copy`)**

- **Objective:** Learn how to explicitly duplicate heap data using `Clone` and how stack-only data uses `Copy`.
- **Activities:**
  - Read: TRPL - Chapter 4.1 "What is Ownership?" (Sections: Ways Variables and Data Interact: Clone, Stack-Only Data: Copy). (20-30 mins)
  - Practice:
    - Use the `.clone()` method to duplicate `String` data.
    - Observe how integers and other simple types (that implement the `Copy` trait) are copied rather than moved.
    - Write a function that takes a `String`, clones it, and returns the clone.
    - Write a function that takes an integer and returns it (observing copy). (30-40 mins)
  - Review: Briefly re-read TRPL Chapter 4.1 to solidify concepts. (10-15 mins)
  - Optional: Continue with Rustlings: `move_semantics` exercises.

**Weekend Project Addition: The Guessing Game from Chapter 2**

- **Objective:** Apply the concepts you've learned by building the guessing game project from TRPL Chapter 2.
- **Activities:**
  - Read: TRPL - Chapter 2 "Programming a Guessing Game" (30-40 mins)
  - Code along:
    - Create a new project: `cargo new guessing_game`
    - Set up a simple guessing game that:
      - Generates a random number
      - Takes user input
      - Compares the user's guess to the random number
      - Gives feedback and allows multiple guesses
    - Learn how to add dependencies to your project via Cargo.toml
    - Practice using the `match` expression for simple error handling
    - Use loops to allow multiple guesses (60-90 mins)
  - This project integrates multiple concepts you've learned: variables, control flow, functions, and introduces new ones like external crates and basic error handling.

---
