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
    // let mut stdout = stdout().into_raw_mode().unwrap();
    //
    // print!("{}{}", clear::All, cursor::Goto(1, 1));
    //
    // print!(
    //     "{}{red}more red than any comrade{reset}\r",
    //     cursor::Goto(1, 1),
    //     red = color::Fg(color::Red),
    //     reset = color::Fg(color::Reset)
    // );
    //
    // stdout.flush().unwrap();
    // // Sleep for a short period of time.
    // thread::sleep(Duration::from_millis(1000));
    // // Go back;
    //
    // // Clear the line and print some new stuff
    // print!(
    //     "{clear}{red}w{blue}a{green}y{red} space communism{reset}",
    //     clear = clear::CurrentLine,
    //     red = color::Fg(color::Red),
    //     blue = color::Fg(color::Blue),
    //     green = color::Fg(color::Green),
    //     reset = color::Fg(color::Reset)
    // );
    //
    // stdout.flush().unwrap();
    //
    // thread::sleep(Duration::from_millis(1000));
    //
    // print!(
    //     "{goto}{clear}better",
    //     goto = cursor::Goto(3, 3),
    //     clear = clear::CurrentLine
    // );
    //
    // stdout.flush().unwrap();
    //
    // thread::sleep(Duration::from_secs(2));

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
