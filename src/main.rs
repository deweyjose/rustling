use crate::health::Health::Alive;
use crate::pattern::Pattern;
use crate::pattern::PatternType;
use clap::Parser;
use std::fs::File;
use std::io;
use std::io::Read;

mod app;
mod commands;
mod coordinates;
mod grid;
mod health;
mod orchestrator;
mod pattern;
mod renderer;
mod size;
mod theme;
mod user_input;
mod viewport;
mod widgets;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to a patterns file
    #[arg(short, long, default_value = "patterns.json")]
    patterns: String,

    /// make the grid N times bigger than the viewport
    #[arg(short = 'm', long, default_value_t = 3)]
    grid_multiplier: usize,

    /// maximum grid width (caps the multiplier calculation)
    #[arg(long)]
    grid_max_width: Option<usize>,

    /// maximum grid height (caps the multiplier calculation)
    #[arg(long)]
    grid_max_height: Option<usize>,
}

fn create_default_pattern() -> Vec<PatternType> {
    vec![PatternType {
        name: String::from("default"),
        patterns: vec![Pattern {
            name: String::from("blinker"),
            matrix: vec![vec![Alive, Alive, Alive]],
            rotation_count: 0,
        }],
    }]
}

fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = crossterm::terminal::disable_raw_mode();
        let mut stdout = std::io::stdout();
        let _ = crossterm::execute!(
            stdout,
            crossterm::event::DisableMouseCapture,
            crossterm::terminal::LeaveAlternateScreen
        );
        original_hook(panic_info);
    }));
}

fn main() -> io::Result<()> {
    install_panic_hook();
    let args = Args::parse();

    let configuration: Vec<PatternType> = if let Ok(mut file) = File::open(&args.patterns) {
        let mut buff = String::new();
        match file.read_to_string(&mut buff) {
            Ok(_) => match serde_json::from_str(&buff) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to parse patterns file '{}': {}",
                        args.patterns, e
                    );
                    eprintln!("Using default pattern instead.");
                    create_default_pattern()
                }
            },
            Err(e) => {
                eprintln!(
                    "Warning: Failed to read patterns file '{}': {}",
                    args.patterns, e
                );
                eprintln!("Using default pattern instead.");
                create_default_pattern()
            }
        }
    } else {
        if args.patterns != "patterns.json" {
            eprintln!(
                "Warning: Could not open patterns file '{}', using default pattern.",
                args.patterns
            );
        }
        create_default_pattern()
    };

    let grid_config = orchestrator::GridConfig {
        multiplier: args.grid_multiplier,
        max_width: args.grid_max_width,
        max_height: args.grid_max_height,
    };

    let mut viewer = orchestrator::Orchestrator::init(configuration, grid_config)?;
    viewer.run()?;
    Ok(())
}
