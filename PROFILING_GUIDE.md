# Algorithm Profiling Guide

This guide shows how to use the `#[profile_algorithm]` procedural macro to automatically benchmark your algorithms.

## Quick Start

### 1. Annotate Your Algorithm

**Choose one of three configuration modes:**

**Mode A: Explicit Sizes** (Manual control)
```rust
use algoprofile_macros::profile_algorithm;

#[profile_algorithm(sizes = [10, 100, 1000, 10000], iterations = 5)]
pub fn my_algorithm(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Tests exactly these sizes: 10, 100, 1000, 10000
}
```

**Mode B: Gradual Range** (Auto step - recommended for smooth plots)
```rust
#[profile_algorithm(range = (1, 100), iterations = 3)]
pub fn my_algorithm_gradual(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Tests from 1 to 100 with auto-calculated step
    // Generates ~20-30 evenly-spaced data points
}
```

**Mode C: Custom Step** (Full control)
```rust
#[profile_algorithm(range = (1, 1000, 50), iterations = 5)]
pub fn my_algorithm_custom(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Tests: 1, 51, 101, 151, ..., 951
}
```

### 2. Call the Auto-Generated Profiler

The macro automatically creates a function named `profile_{function_name}`:

```rust
fn main() {
    // Call the auto-generated profiler
    let performance_data = profile_my_algorithm();

    // Returns: Vec<(usize, Duration)>
    // - usize: input size
    // - Duration: average execution time
}
```

## What the Macro Does

1. **Preserves Your Original Function** - Your function works exactly as before
2. **Generates a Profiler Function** - Creates `profile_{function_name}()`
3. **Benchmarks Multiple Input Sizes** - Tests across your specified range or sizes
4. **Runs Multiple Iterations** - Default: 5 iterations per size for accuracy
5. **Outputs Results** - Prints formatted table and CSV data
6. **Returns Data** - Returns `Vec<(usize, Duration)>` for plotting

## Configuration Options

### Input Size Configuration

| Mode | Syntax | Use Case | Example Output |
|------|--------|----------|----------------|
| **Explicit** | `sizes = [10, 100, 1000]` | Specific sizes needed | Tests exactly: 10, 100, 1000 |
| **Gradual** | `range = (1, 100)` | Smooth gradual analysis | Tests: 1, 11, 21, 31, ..., 91 |
| **Custom Step** | `range = (start, end, step)` | Fine-grained control | Tests: start, start+step, ... |

### Auto Step Calculation (Gradual Mode)

When using `range = (start, end)`, the macro automatically calculates an optimal step size:

- Range ≤ 30: step = 1 (test every value)
- Range ≤ 300: step = 10
- Range ≤ 3000: step = 100
- Range > 3000: step = 1000

This ensures you get ~20-30 data points for smooth plots without overwhelming detail.

### Iterations Parameter

```rust
#[profile_algorithm(range = (1, 100), iterations = 10)]
```

Controls how many times each input size is tested. Higher values = more accurate averages but longer runtime.

## Output Format

### Console Output

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

=== Profile Data (CSV Format) ===
input_size,time_micros
10,8
100,80
500,497
1000,537
5000,1304
10000,2683
```

## Creating Performance Plots

Use the returned data with `plotters` crate:

```rust
use plotters::prelude::*;

fn generate_plot(data: Vec<(usize, Duration)>, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_size = data.iter().map(|(s, _)| *s).max().unwrap_or(1);
    let max_time = data.iter().map(|(_, d)| d.as_micros()).max().unwrap_or(1);

    let mut chart = ChartBuilder::on(&root)
        .caption("Algorithm Performance", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0..max_size, 0..max_time as usize)?;

    chart
        .configure_mesh()
        .x_desc("Input Size (n)")
        .y_desc("Time (microseconds)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        data.iter().map(|(s, d)| (*s, d.as_micros() as usize)),
        &BLUE,
    ))?;

    root.present()?;
    Ok(())
}

fn main() {
    let data = profile_my_algorithm();
    generate_plot(data, "performance.png").unwrap();
}
```

## Customizing Test Data

Currently, the macro generates sequential test data:
```rust
let nums: Vec<i32> = (0..size).map(|i| i as i32).collect();
let target = (size - 1) as i32;
```

For algorithms requiring different input patterns, you can:
1. Create wrapper functions with custom data generation
2. Modify the macro in `algoprofile_macros/src/lib.rs`

## Understanding the Results

### Time Complexity Analysis

By plotting input size vs. execution time, you can visually verify time complexity:

- **O(1) - Constant**: Flat horizontal line
- **O(log n) - Logarithmic**: Slow upward curve
- **O(n) - Linear**: Straight diagonal line
- **O(n log n)**: Slightly curved upward
- **O(n²) - Quadratic**: Sharp exponential curve

### Statistical Accuracy

The macro runs multiple iterations (default: 5) per input size to:
- Reduce variance from system noise
- Account for CPU caching effects
- Provide more reliable average timings

## Example: Two Sum Algorithm

```rust
#[profile_algorithm(sizes = [10, 100, 1000, 10000], iterations = 5)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j as i32, i as i32];
        }
        map.insert(*num, i);
    }
    vec![]
}

fn main() {
    let data = profile_two_sum();
    generate_plot(data, "two_sum_performance.png").unwrap();
}
```

The resulting plot shows O(n) linear time complexity, confirming the algorithm's efficiency.

## Tips

1. **Warm-up runs**: First iteration may be slower due to cold caches
2. **Large inputs**: Be careful with memory-intensive algorithms
3. **System load**: Close other applications for more accurate results
4. **CSV export**: Copy CSV output for analysis in Excel/Python
5. **Comparative analysis**: Profile multiple algorithms and overlay plots

## Future Enhancements

Potential additions to the macro:
- Configurable input size ranges via macro attributes
- Custom data generator functions
- Memory usage profiling
- Multi-threaded benchmarking
- Automatic complexity class detection
