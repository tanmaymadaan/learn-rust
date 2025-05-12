### Week 8: Modules, Crates & Cargo In-depth

**Goal:** Learn to effectively organize large Rust projects using modules, understand the crate ecosystem, and become more proficient with Cargo's features.

**Total Time:** ~3.5 - 5.5 hours (Weekdays: 5 _ 20-30 min, Weekend: 2 _ 60-90 min)

**Key Resources:**

- "The Rust Programming Language" (TRPL) book: Chapter 7 ("Managing Growing Projects with Packages, Crates, and Modules"), Chapter 14 ("More About Cargo and Crates.io").
- Rustlings exercises (e.g., `modules`, `cargo`).

---

**Day 50 (Weekday - 20-30 mins): Packages and Crates & Defining Modules**

- **Objective:** Understand the distinction between packages and crates, and learn the basics of defining modules.
- **Activities:**
  - Read: TRPL - Chapter 7.1 "Packages and Crates for Code Reuse" and Chapter 7.2 "Defining Modules to Control Scope and Privacy". (15-20 mins)
  - Practice:
    - Identify the crate root for a binary and a library package.
    - Define a simple module within `src/main.rs` or `src/lib.rs`. (5-10 mins)

**Day 51 (Weekday - 20-30 mins): Paths for Referring to Items in Modules & `super`**

- **Objective:** Learn how to use paths to access items in different modules and use `super` to access items in a parent module.
- **Activities:**
  - Read: TRPL - Chapter 7.3 "Paths for Referring to an Item in the Module Tree". (15-20 mins)
  - Practice:
    - Create a nested module structure.
    - Call functions and use types from different modules using their paths.
    - Use `super` to call a function in a parent module. (5-10 mins)

**Day 52 (Weekday - 20-30 mins): Controlling Privacy with `pub` & The `use` Keyword**

- **Objective:** Understand how to make items public using `pub` and how to bring paths into scope with `use`.
- **Activities:**
  - Read: TRPL - Chapter 7.3 (re-read "Exposing Paths with the `pub` Keyword") and Chapter 7.4 "Bringing Paths into Scope with the `use` Keyword". (15-20 mins)
  - Practice:
    - Use `pub` to make modules, functions, structs, and enums public.
    - Use `use` to bring items into the current scope and call them more concisely.
    - Experiment with `pub use` to re-export items. (5-10 mins)

**Day 53 (Weekday - 20-30 mins): Separating Modules into Different Files**

- **Objective:** Learn how to organize modules across multiple files for better project structure.
- **Activities:**
  - Read: TRPL - Chapter 7.5 "Separating Modules into Different Files". (15-20 mins)
  - Practice:
    - Take a project with inline modules and refactor it to use separate files for submodules (e.g., `src/module_name.rs` and `src/module_name/submodule.rs`). (5-10 mins)

**Day 54 (Weekday - 20-30 mins): Cargo.toml & Crates.io Overview**

- **Objective:** Get an overview of `Cargo.toml` manifest options and how to find and use crates from `crates.io`.
- **Activities:**
  - Read: TRPL - Chapter 14.1 "Customizing Builds with Release Profiles" (briefly, focus on what profiles are).
  - Read: TRPL - Chapter 14.3 "Cargo Workspaces" (briefly, understand the concept).
  - Read: TRPL - Chapter 2.4 "Using a Crate from Crates.io" (if not covered before, or as a refresher).
  - Explore: Browse `crates.io`. Find a simple, useful crate (e.g., `rand` for random numbers, or `chrono` for dates/times). Add it as a dependency to a test project and use a basic function from it. (20-30 mins)

**Day 55 (Weekend - 60-90 mins): Cargo Features & Conditional Compilation**

- **Objective:** Understand how to use Cargo features for conditional compilation.
- **Activities:**
  - Read: The "Features" section of the Cargo Book (linked from TRPL Ch 14, or search "Cargo Book Features"). (30-40 mins)
  - Practice:
    - Define a feature in your `Cargo.toml`.
    - Use `#[cfg(feature = "your_feature")]` to conditionally compile some code.
    - Build with and without the feature enabled. (30-40 mins)
  - Review module system concepts from TRPL Chapter 7.

**Day 56 (Weekend - 60-90 mins): Publishing a Crate (Conceptual) & Rustlings**

- **Objective:** Understand the process of publishing a crate to `crates.io` (you don't have to actually publish). Consolidate module and Cargo knowledge.
- **Activities:**
  - Read: TRPL - Chapter 14.2 "Publishing a Crate to Crates.io". Focus on the steps and metadata. (20-30 mins)
  - Practice Project: Structure a small project with multiple modules, some public, some private. Include an external dependency.
  - Rustlings: Work through `modules` exercises. (30-40 mins)
  - Review: Key concepts from TRPL Chapters 7 and 14.

---
