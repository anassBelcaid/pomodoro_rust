# Pomodoro CLI Tool

A command-line interface (CLI) tool for managing Pomodoro sessions, written in Rust. This tool allows users to set custom Pomodoro durations, logs each session into an SQL database, and provides statistical analysis of your productivity.

## Features

- **Customizable Pomodoro Duration**: Set the duration of each Pomodoro session via command line arguments.
- **Data Persistence**: All session data is stored in an SQL database for long-term tracking.
- **Productivity Analysis**: Generates statistical graphs showing productivity trends over time.

## Using `clap` for Argument Parsing

To make the CLI user-friendly and robust, we utilize the `clap` library to handle command line arguments. Currently, the tool supports a single command line argument:

- `--duration`: Specifies the duration of the Pomodoro session in minutes.

### Implementation

Here's how the `duration` argument is set up using `clap`:

```rust
use clap::{App, Arg};

fn main() {
    let matches = App::new("Pomodoro CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Manages Pomodoro sessions")
        .arg(Arg::with_name("duration")
            .short('d')
            .long("duration")
            .help("Sets the Pomodoro session duration in minutes")
            .takes_value(true)
            .required(false)
            .default_value("25"))
        .get_matches();

    let duration: i32 = matches.value_of("duration").unwrap().parse().unwrap();
    println!("Pomodoro session duration: {} minutes", duration);
}
```

## Development Plan

- [x] **First Argument**: Implement the `duration` argument with `clap`. *(Partially done)*
- [ ] **SQL Storage**: Integrate SQL database to store session results.
- [ ] **Statistics**: Generate and display productivity graphs based on stored data.
