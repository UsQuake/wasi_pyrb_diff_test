#[cfg(test)]
mod tests {
    use crate::grammar_fuzzer::*;
    #[test]

    fn test_is_valid_grammar() {
        assert!(dot_escape(&"hello".to_string(), None) == "hello");
        assert!(dot_escape(&"<hello>, world".to_string(), None) == "\\<hello\\>\\, world");
        assert!(dot_escape(&"\\n".to_string(), None) == "\\\\n");
        assert!(dot_escape(&"\n".to_string(), Some(false)) == "\\\\n");
        assert!(dot_escape(&"\n".to_string(), Some(true)) == "\\\\n (10)");
        assert!(dot_escape(&"\n".to_string(), Some(true)) == "\\\\n (10)");
        assert!(dot_escape(&"\x01".to_string(), Some(false)) == "\\\\x01");
        assert!(dot_escape(&"\x01".to_string(), None) == "\\\\x01 (1)");
    }
}