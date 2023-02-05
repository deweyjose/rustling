use crate::health::Health::Alive;
use crate::pattern::Pattern;
use crate::pattern::PatternType;
use std::env;
use std::fs::File;
use std::io::Read;

mod coordinates;
mod grid;
mod grid_input;
mod grid_viewer;
mod health;
mod pattern;
mod size;

fn main() {
    let args: Vec<String> = env::args().collect();
    let configuration: Vec<PatternType> =
        if let Ok(mut file) = File::open(args.get(1).unwrap_or(&String::from("patterns.json"))) {
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

    let mut viewer = grid_viewer::init(configuration);
    viewer.render();
    viewer.run();
}
