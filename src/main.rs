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
    let mut file = File::open("shapes.json").unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let configuration: Vec<pattern::PatternType> = serde_json::from_str(&buff).unwrap();

    let mut viewer = grid_viewer::init(configuration);
    viewer.render();
    viewer.run();
}
