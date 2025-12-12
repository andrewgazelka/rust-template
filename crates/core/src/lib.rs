//! {{project-name}} core library

/// Returns a greeting message.
#[must_use]
pub fn hello() -> &'static str {
    "Hello from {{project-name}}!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert!(hello().contains("Hello"));
    }
}
