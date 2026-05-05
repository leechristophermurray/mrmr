//! mrmr-app -- application logic for `MurMur`
//! Licensed under Apache-2.0
use std::io::{self, Write};

/// Prints a greeting to the provided name.
///
/// # Errors
///
/// This function will return an error if writing to the standard output stream fails.
pub fn hello(name: &str) -> Result<(), io::Error> {
    writeln!(std::io::stdout(), "Hello, {name}!")?;
    let family_name = mrmr_core::protocol_family_names();
    _ = writeln!(std::io::stdout(), "mrmr protocol family: `{family_name}`");
    Ok(())
}

/// Prints a simple greeting.
///
/// # Errors
///
/// This function will return an error if writing to the standard output stream fails.
pub fn hi() -> Result<(), io::Error> {
    writeln!(std::io::stdout(), "Hi!")?;
    Ok(())
}
