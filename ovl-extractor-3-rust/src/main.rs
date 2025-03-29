mod ovl;
mod gui;

fn main() {
    env_logger::init();
    if let Err(e) = gui::run_gui() {
        eprintln!("Error running application: {}", e);
    }
}
