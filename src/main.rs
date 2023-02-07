use crate::health::Health::Alive;
use crate::pattern::Pattern;
use crate::pattern::PatternType;
use clap::{arg, Parser};
use std::fs::File;
use std::io::Read;

mod coordinates;
mod grid;
mod grid_input;
mod grid_viewer;
mod health;
mod pattern;
mod size;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to a patterns file
    #[arg(short, long, default_value = "patterns.json")]
    patterns: String,

    /// make the grid N times bigger than the view port
    #[arg(short, long, default_value_t = 3)]
    multiplier: usize,
}

fn main() {
    let args = Args::parse();

    let configuration: Vec<PatternType> = if let Ok(mut file) = File::open(args.patterns) {
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();
        serde_json::from_str(&buff).unwrap()
    } else {
        vec![PatternType {
            name: String::from("default"),
            patterns: vec![Pattern {
                name: String::from("default"),
                matrix: vec![vec![Alive, Alive, Alive]],
            }],
        }]
    };

    let mut viewer = grid_viewer::init(configuration, args.multiplier);
    viewer.render();
    viewer.run();
}
