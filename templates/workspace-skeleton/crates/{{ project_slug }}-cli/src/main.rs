// use {{ project_slug }}_app;
// use {{ project_slug }}_core;
use std::io::Write;

fn main() {
    if let Some(name) = std::env::args().nth(1) {
        _ = {{ crate_prefix }}_app::hello(&name);
    } else {
        _ = {{ crate_prefix }}_app::hi();
        _ = writeln!(
            std::io::stdout(),
            "Sorry, I can't tell you the protocol family name."
        );
    }
}
