# AlgoProfile CLI Documentation

Command-line interface for algorithm profiling and visualization.

## Installation

```bash
cargo build --release
```

## Usage

```bash
algoprofile <COMMAND> [OPTIONS]
```

## Commands

### `list`
List all available algorithms with complexity and description.

**Usage:**
```bash
algoprofile list
```

**Example Output:**
```
📚 Available Algorithms:

Name                 Complexity      Description
───────────────────────────────────────────────────────────────────────────
two_sum              O(n)            Find two numbers that sum to target
```

---

### `run`
Run a specific algorithm with sample data.

**Usage:**
```bash
algoprofile run <ALGORITHM>
```

**Arguments:**
- `<ALGORITHM>` - Name of the algorithm to run (e.g., `two_sum`)

**Example:**
```bash
algoprofile run two_sum
```

**Output:**
```
🚀 Running algorithm: two_sum

[TIMED] solve_two_sum took 101.25µs
Input: [2, 7, 11, 15], Target: 9
Result: [0, 1]
```

---

### `profile`
Profile an algorithm and visualize time & memory complexity.

**Usage:**
```bash
algoprofile profile <ALGORITHM> [OPTIONS]
```

**Arguments:**
- `<ALGORITHM>` - Name of the algorithm to profile

**Options:**
- `-i, --interactive` - Launch interactive plot immediately after profiling

**Examples:**

Profile with ASCII plot:
```bash
algoprofile profile two_sum
```

Profile with interactive visualization:
```bash
algoprofile profile two_sum --interactive
```

**Output:**
- ASCII plot showing time and memory complexity (cyan and green lines)
- Normalized 0-100% scale
- Performance metrics and complexity analysis
- Optional interactive TUI with zoom/pan controls

---

## Global Options

- `-h, --help` - Print help information
- `-V, --version` - Print version information

## Interactive Plot Controls

When using `--interactive` flag:

| Key | Action |
|-----|--------|
| `←` / `→` | Pan left/right |
| `↑` / `↓` | Pan up/down |
| `+` / `=` | Zoom in |
| `-` / `_` | Zoom out |
| `Tab` | Select next data point |
| `Shift+Tab` | Select previous data point |
| `R` | Reset view |
| `Q` / `Esc` | Quit |

## Examples

List all algorithms:
```bash
algoprofile list
```

Run two_sum with sample data:
```bash
algoprofile run two_sum
```

Profile with ASCII visualization:
```bash
algoprofile profile two_sum
```

Profile with interactive mode:
```bash
algoprofile profile two_sum -i
```

Get help for specific command:
```bash
algoprofile profile --help
```

## Output Interpretation

### ASCII Plot
- **Cyan dots (·)** - Time complexity
- **Green dots (·)** - Memory complexity
- **Y-axis** - Normalized scale (0-100%)
- **X-axis** - Input size (n)

### Metrics
- **Time Complexity** - Average execution time across multiple iterations
- **Memory Complexity** - Estimated memory usage (input + data structures)
- **Normalization** - Both metrics scaled to 0-100% for comparison

## Adding New Algorithms

1. Create new directory in `algoprofile/src/algorithms/<algo_name>/`
2. Implement algorithm with `#[profile_algorithm]` macro
3. Update `list_algorithms()` and `run_algorithm()` in `main.rs`
4. Algorithm will automatically appear in CLI

## Troubleshooting

**Algorithm not found:**
- Use `algoprofile list` to see available algorithms
- Check spelling matches exactly (case-sensitive)

**Interactive mode not working:**
- Ensure terminal supports ANSI colors and escape sequences
- Try running in a modern terminal (iTerm2, Terminal.app, etc.)

**Build errors:**
- Run `cargo clean && cargo build`
- Ensure Rust toolchain is up to date: `rustup update`

## Version

AlgoProfile v0.1.0
