# AlgoProfile

A Rust project demonstrating algorithm implementations with procedural macros.

## Project Structure

This is a workspace with two crates:

```
AlgoProfile/
├── Cargo.toml                  # Workspace root
├── algoprofile/                # Main binary crate
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── algorithms/
│           └── hash_maps/
│               ├── mod.rs
│               └── two_sum.rs
└── algoprofile_macros/         # Procedural macros crate
    ├── Cargo.toml
    └── src/
        └── lib.rs
```

## Procedural Macros

The `algoprofile_macros` crate provides four types of procedural macros:

### 1. Derive Macro: `#[derive(AlgoDebug)]`
Automatically implements `Display` trait for structs.

```rust
#[derive(AlgoDebug)]
struct Algorithm {
    name: String,
    complexity: String,
}
```

### 2. Attribute Macro: `#[timed]`
Adds execution time logging to functions.

```rust
#[timed]
fn solve_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    two_sum(nums, target)
}
```

### 3. Attribute Macro: `#[profile_algorithm]` ⭐ NEW
**Automatically profiles algorithms with varying input sizes and generates performance data.**

This macro:
- Benchmarks your algorithm across multiple input sizes
- Runs multiple iterations (default: 5) for statistical accuracy
- Auto-generates a profiler function named `profile_{function_name}`
- Outputs timing data in both table and CSV format
- Perfect for visualizing algorithm performance

**Three ways to specify input sizes:**

**Option 1: Explicit sizes**
```rust
#[profile_algorithm(sizes = [10, 100, 1000, 10000], iterations = 5)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Your algorithm implementation
}
```

**Option 2: Gradual range (auto step calculation)**
```rust
#[profile_algorithm(range = (1, 100), iterations = 3)]
pub fn two_sum_gradual(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Tests with sizes: 1, 11, 21, 31, ..., 91
    // Step is auto-calculated for ~20-30 data points
}
```

**Option 3: Custom step size**
```rust
#[profile_algorithm(range = (1, 1000, 50), iterations = 5)]
pub fn two_sum_custom(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Tests with sizes: 1, 51, 101, 151, ..., 951
}
```

All variants automatically generate a `profile_{function_name}()` function that returns `Vec<(usize, Duration)>`.

**Output:**
```
=== Profiling two_sum ===
Input Size   Avg Time             Total Time
----------------------------------------------------
10           8.308µs              41.543µs
100          80.491µs             402.458µs
500          497.616µs            2.488084ms
1000         537.808µs            2.689041ms
5000         1.304716ms           6.523583ms
10000        2.683666ms           13.418333ms
```

### 4. Function-like Macro: `benchmark!`
Creates a benchmark helper for quick performance testing.

```rust
benchmark!(some_function());
```

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## IDE Setup (RustRover/IntelliJ IDEA)

Procedural macros can sometimes cause IDE analysis issues. If you see errors like "Cannot find struct, variant, or union type" even though the code compiles fine, try these solutions:

### Solution 1: Invalidate Caches (Most Effective)
1. Go to `File` → `Invalidate Caches...`
2. Select `Invalidate and Restart`
3. Wait for the IDE to re-index the project

### Solution 2: Reload Cargo Project
1. Right-click on `Cargo.toml` in the project view
2. Select `Reload Cargo Project`

### Solution 3: Configure Rust Settings
1. Go to `Settings` → `Languages & Frameworks` → `Rust`
2. Enable "Use offline cargo check"
3. Enable "Expand declarative macros" (if available)

### Solution 4: Update Rust Plugin
1. Go to `Settings` → `Plugins`
2. Check for updates to the Rust plugin
3. Restart IDE after updating

### Solution 5: Clean Rebuild
```bash
cargo clean && cargo build
```

### Workaround
If IDE errors persist but code compiles, you can suppress warnings:
```rust
#[allow(dead_code)]  // IDE may not recognize proc macro expansion
#[derive(AlgoDebug)]
struct Algorithm { ... }
```

**Note**: These are IDE-only issues. The code will always compile and run correctly even if the IDE shows errors.

## Adding New Macros

1. Add your macro implementation in `algoprofile_macros/src/lib.rs`
2. Export it with `#[proc_macro]`, `#[proc_macro_derive]`, or `#[proc_macro_attribute]`
3. Use it in `algoprofile/src/main.rs`

## Visualizing Performance

The project includes dual visualization outputs:

### 1. Terminal Plot (ASCII Art)
Instant visualization directly in your terminal using Braille characters for high resolution:

```
📊 Two Sum Algorithm Performance - Terminal Plot
⡁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⠁⠈⠀⢁⣈⠤⠕⠊⠀⠁⠈⠀⡁
⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⠤⠔⠒⠉⠁⠀⠀⠀⠀⠀⠀⠀⠄
⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡠⠤⠔⠒⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠂
...
💡 The linear trend confirms O(n) time complexity
```

### 2. PNG File Export
High-quality plot saved as `two_sum_performance.png` for documentation and presentations.

Both outputs clearly show the O(n) linear time complexity of the Two Sum algorithm.

## Dependencies

**Procedural Macros:**
- `syn` 2.0 - Parsing Rust code
- `quote` 1.0 - Generating Rust code
- `proc-macro2` 1.0 - Procedural macro utilities

**Main Application:**
- `plotters` 0.3 - PNG plot generation
- `textplots` 0.8 - Terminal ASCII art plotting
