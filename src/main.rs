mod text_viewer;

fn main() {
    let mut viewer = text_viewer::init();
    viewer.render();
    viewer.run();
}
