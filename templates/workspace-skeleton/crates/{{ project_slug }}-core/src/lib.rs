//! {{ project_slug }}-core -- pure domaine types for `{{ project_canonical_name }}`
//! Licensed under Apache-2.0

/// Returns the protocol family name. PLACEHOLDER UNTIL W2
#[must_use]
pub const fn protocol_family_names() -> &'static str {
    "{{ project_slug }}"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_family_name_is_{{ crate_prefix }}() {
        let result = protocol_family_names();
        assert_eq!(result, "{{ crate_prefix }}");
    }
}
