// Domain layer
pub mod domain;

// Application layer
pub mod application;

// Infrastructure layer
pub mod infrastructure;

// Presentation layer
pub mod presentation;

#[cfg(test)]
mod tests {
    #[test]
    fn lib_compiles() {
        // Basic test to ensure lib compiles
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn ascii_chars_exist() {
        // Test that we have some basic ASCII functionality concepts
        let ascii_chars = ['@', '#', '*', '.', ' '];
        assert!(!ascii_chars.is_empty());
    }
}
