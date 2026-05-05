//! mrmr-core -- pure domaine types for `MurMur`
//! Licensed under Apache-2.0

/// Returns the protocol family name. PLACEHOLDER UNTIL W2
#[must_use]
pub const fn protocol_family_names() -> &'static str {
    "mrmr"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_family_name_is_mrmr() {
        let result = protocol_family_names();
        assert_eq!(result, "mrmr");
    }
}
