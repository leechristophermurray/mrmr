// use mrmr_app;
// use mrmr_core;
use std::io::Write;

fn main() {
    if let Some(name) = std::env::args().nth(1) {
        _ = mrmr_app::hello(&name);
    } else {
        _ = mrmr_app::hi();
        _ = writeln!(
            std::io::stdout(),
            "Sorry, I can't tell you the protocol family name."
        );
    }
}
