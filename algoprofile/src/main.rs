mod algorithms;
mod cli;

use algoprofile_macros::{timed, AlgoDebug};
use algorithms::two_sum::algo::{algo, profile_algo_gradual};

// use textplots::{Chart, Plot, Shape}; // Not needed - using custom plotter
use crate::cli::start_cli;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{Axis, Block, Borders, Chart as RatatuiChart, Dataset, GraphType},
    Terminal,
};
use std::path::Path;
use std::{fs, io};

// Example struct using the derive macro
#[derive(AlgoDebug)]
struct Algorithm {
    name: String,
    complexity: String,
}

// Example function using the attribute macro for timing
#[timed]
fn solve_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    algo(nums, target)
}

fn generate_terminal_plot(data: &[(usize, std::time::Duration, usize)]) {
    println!("\n{}", "=".repeat(80));
    println!("📊 Algorithm Performance - Time & Memory Complexity (Normalized)");
    println!("{}", "=".repeat(80));

    let max_x = data.iter().map(|(size, _, _)| *size).max().unwrap_or(100) as f32;

    // Get max values for normalization
    let max_time = data
        .iter()
        .map(|(_, d, _)| d.as_micros())
        .max()
        .unwrap_or(100) as f32;
    let max_memory = data.iter().map(|(_, _, m)| *m).max().unwrap_or(100) as f32;

    // Normalize both to 0-100 scale
    let time_points: Vec<(f32, f32)> = data
        .iter()
        .map(|(size, duration, _)| {
            let normalized = (duration.as_micros() as f32 / max_time) * 100.0;
            (*size as f32, normalized)
        })
        .collect();

    let memory_points: Vec<(f32, f32)> = data
        .iter()
        .map(|(size, _, memory)| {
            let normalized = (*memory as f32 / max_memory) * 100.0;
            (*size as f32, normalized)
        })
        .collect();

    // ANSI color codes
    const CYAN: &str = "\x1b[36m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";
    const GRAY: &str = "\x1b[90m";

    println!(
        "\n{}Combined Plot - Both complexities overlaid:{}",
        YELLOW, RESET
    );

    // Custom ASCII plotter with color support
    let width = 120;
    let height = 30;

    #[derive(Clone, Copy)]
    enum PlotColor {
        Cyan,
        Green,
    }

    // Create empty grid
    let mut grid: Vec<Vec<Option<(char, PlotColor)>>> = vec![vec![None; width]; height];

    // Helper function to plot a line with color
    let plot_line = |grid: &mut Vec<Vec<Option<(char, PlotColor)>>>,
                     points: &[(f32, f32)],
                     color: PlotColor,
                     marker: char| {
        for i in 0..points.len().saturating_sub(1) {
            let (x1, y1) = points[i];
            let (x2, y2) = points[i + 1];

            // Map to grid coordinates
            let grid_x1 = ((x1 / max_x) * width as f32).min((width - 1) as f32) as usize;
            let grid_y1 = height
                .saturating_sub(1)
                .saturating_sub(((y1 / 100.0) * height as f32) as usize);
            let grid_x2 = ((x2 / max_x) * width as f32).min((width - 1) as f32) as usize;
            let grid_y2 = height
                .saturating_sub(1)
                .saturating_sub(((y2 / 100.0) * height as f32) as usize);

            // Draw line between points using Bresenham's algorithm
            let dx = (grid_x2 as i32 - grid_x1 as i32).abs();
            let dy = (grid_y2 as i32 - grid_y1 as i32).abs();
            let sx = if grid_x1 < grid_x2 { 1 } else { -1 };
            let sy = if grid_y1 < grid_y2 { 1 } else { -1 };
            let mut err = dx - dy;

            let mut x = grid_x1 as i32;
            let mut y = grid_y1 as i32;

            loop {
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    grid[y as usize][x as usize] = Some((marker, color));
                }

                if x == grid_x2 as i32 && y == grid_y2 as i32 {
                    break;
                }

                let e2 = 2 * err;
                if e2 > -dy {
                    err -= dy;
                    x += sx;
                }
                if e2 < dx {
                    err += dx;
                    y += sy;
                }
            }
        }
    };

    // Plot both lines with small pixel markers
    plot_line(&mut grid, &time_points, PlotColor::Cyan, '·');
    plot_line(&mut grid, &memory_points, PlotColor::Green, '·');

    // Print the grid
    println!("\n  100% {}", GRAY);
    for row in &grid {
        print!("       {}│{}", GRAY, RESET);
        for cell in row {
            if let Some((marker, color)) = cell {
                let color_code = match color {
                    PlotColor::Cyan => CYAN,
                    PlotColor::Green => GREEN,
                };
                print!("{}{}{}", color_code, marker, RESET);
            } else {
                print!(" ");
            }
        }
        println!();
    }
    print!("     0% {}└", GRAY);
    for _ in 0..width {
        print!("─");
    }
    println!("{}", RESET);
    println!(
        "        0{:>width$}",
        format!("{:.0}", max_x),
        width = width - 1
    );

    println!("\n📈 Legend:");
    println!(
        "   {}· Time Complexity{} - Normalized (max: {:.2}μs)",
        CYAN, RESET, max_time
    );
    println!(
        "   {}· Memory Complexity{} - Normalized (max: {} bytes)",
        GREEN, RESET, max_memory as usize
    );
    println!("   Y-axis: Normalized scale (0-100%)");
    println!("\n💡 Both time and memory show O(n) linear complexity");
    println!("{}", "=".repeat(80));
}

struct InteractivePlotState {
    data: Vec<(usize, std::time::Duration, usize)>,
    zoom: f64,
    pan_x: f64,
    pan_y: f64,
    selected_point: usize,
}

impl InteractivePlotState {
    fn new(data: Vec<(usize, std::time::Duration, usize)>) -> Self {
        Self {
            data,
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            selected_point: 0,
        }
    }

    fn zoom_in(&mut self) {
        self.zoom *= 1.2;
    }

    fn zoom_out(&mut self) {
        self.zoom /= 1.2;
        if self.zoom < 0.1 {
            self.zoom = 0.1;
        }
    }

    fn pan_left(&mut self) {
        self.pan_x -= 0.1 / self.zoom;
    }

    fn pan_right(&mut self) {
        self.pan_x += 0.1 / self.zoom;
    }

    fn pan_up(&mut self) {
        self.pan_y += 0.1 / self.zoom;
    }

    fn pan_down(&mut self) {
        self.pan_y -= 0.1 / self.zoom;
    }

    fn select_next(&mut self) {
        if self.selected_point < self.data.len() - 1 {
            self.selected_point += 1;
        }
    }

    fn select_prev(&mut self) {
        if self.selected_point > 0 {
            self.selected_point -= 1;
        }
    }

    fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan_x = 0.0;
        self.pan_y = 0.0;
    }
}

fn generate_interactive_plot(
    data: Vec<(usize, std::time::Duration, usize)>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = InteractivePlotState::new(data);

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(3),
                ])
                .split(f.area());

            let max_x = state.data.iter().map(|(s, _, _)| *s).max().unwrap_or(100) as f64;

            // Get max values for normalization
            let max_time = state.data.iter().map(|(_, d, _)| d.as_micros()).max().unwrap_or(100) as f64;
            let max_memory = state.data.iter().map(|(_, _, m)| *m).max().unwrap_or(100) as f64;

            // Normalize both to 0-100 scale for display
            let time_data: Vec<(f64, f64)> = state.data
                .iter()
                .map(|(size, duration, _)| {
                    let normalized = (duration.as_micros() as f64 / max_time) * 100.0;
                    (*size as f64, normalized)
                })
                .collect();

            let memory_data: Vec<(f64, f64)> = state.data
                .iter()
                .map(|(size, _, memory)| {
                    let normalized = (*memory as f64 / max_memory) * 100.0;
                    (*size as f64, normalized)
                })
                .collect();

            // Calculate visible bounds based on zoom and pan
            let max_y = 100.0; // Normalized scale
            let center_x = max_x / 2.0 + state.pan_x * max_x;
            let center_y = max_y / 2.0 + state.pan_y * max_y;
            let width = max_x / state.zoom;
            let height = max_y / state.zoom;

            let x_min = (center_x - width / 2.0).max(0.0);
            let x_max = (center_x + width / 2.0).min(max_x * 1.1);
            let y_min = (center_y - height / 2.0).max(0.0);
            let y_max = (center_y + height / 2.0).min(max_y * 1.1);

            let datasets = vec![
                Dataset::default()
                    .name("Time Complexity")
                    .marker(symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Color::Cyan))
                    .data(&time_data),
                Dataset::default()
                    .name("Memory Complexity")
                    .marker(symbols::Marker::Braille)
                    .graph_type(GraphType::Line)
                    .style(Style::default().fg(Color::Green))
                    .data(&memory_data),
            ];

            let selected = &state.data[state.selected_point];
            let chart = RatatuiChart::new(datasets)
                .block(
                    Block::default()
                        .title(format!(
                            "📊 Algorithm Performance | Point: ({}, {:.2}μs, {} bytes) | Zoom: {:.1}x",
                            selected.0,
                            selected.1.as_micros(),
                            selected.2,
                            state.zoom
                        ))
                        .borders(Borders::ALL)
                )
                .x_axis(
                    Axis::default()
                        .title("Input Size (n)")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([x_min, x_max])
                )
                .y_axis(
                    Axis::default()
                        .title("Normalized Scale (0-100%)")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([y_min, y_max])
                );

            f.render_widget(chart, chunks[0]);

            // Help text
            let help = Line::from(vec![
                Span::styled("Controls: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("←/→ Pan X | ↑/↓ Pan Y | +/- Zoom | Tab/Shift+Tab Select Point | R Reset | Q Quit"),
            ]);
            let help_block = Block::default().borders(Borders::ALL);
            f.render_widget(
                ratatui::widgets::Paragraph::new(help).block(help_block),
                chunks[1]
            );
        })?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                    KeyCode::Char('+') | KeyCode::Char('=') => state.zoom_in(),
                    KeyCode::Char('-') | KeyCode::Char('_') => state.zoom_out(),
                    KeyCode::Left => state.pan_left(),
                    KeyCode::Right => state.pan_right(),
                    KeyCode::Up => state.pan_up(),
                    KeyCode::Down => state.pan_down(),
                    KeyCode::Tab => state.select_next(),
                    KeyCode::BackTab => state.select_prev(),
                    KeyCode::Char('r') | KeyCode::Char('R') => state.reset_view(),
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn launch() {
    println!("=== Procedural Macro Examples ===\n");

    // 1. Derive macro example
    let algorithm = Algorithm {
        name: "Two Sum".to_string(),
        complexity: "O(n)".to_string(),
    };
    println!("Derive macro: {}\n", algorithm);

    // 2. Attribute macro example (timed function)
    println!("Attribute macro:");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = solve_two_sum(nums, target);
    println!("Two sum result: {:?}\n", result);

    // 3. Regular call for comparison
    println!("Regular algo call:");
    let result2 = algo(vec![3, 2, 4], 6);
    println!("Result: {:?}\n", result2);

    // 5. Profile with gradual range (1 to 100)
    println!("\n=== Algorithm Profiling: Gradual Range (1 to 100) ===\n");
    let gradual_data = profile_algo_gradual();
    generate_terminal_plot(&gradual_data);

    // Interactive plot with ratatui - loop until user exits
    loop {
        print!("\n🎮 Launch interactive plot? (Y/N): ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("y") || input.trim().eq_ignore_ascii_case("yes") {
            match generate_interactive_plot(gradual_data.clone()) {
                Ok(_) => println!("\n✓ Interactive plot closed"),
                Err(e) => eprintln!("\n✗ Error in interactive plot: {}", e),
            }
            // Loop continues - ask again
        } else {
            println!("Exiting program.");
            break;
        }
    }
}



fn main() {
    start_cli();
}
