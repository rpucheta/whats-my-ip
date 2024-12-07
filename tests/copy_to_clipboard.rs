use whats_my_ip::infra::services::clipboard::copy_to_clipboard;

#[cfg(test)]
mod clipboard_tests {
    use super::*;

    #[test]
    fn test_copy_to_clipboard_success() {
        let text = "Test content";
        let result = copy_to_clipboard(text);
        assert!(result, "Expected clipboard copy to succeed, but it failed.");
    }

    #[test]
    fn test_clipboard_round_trip() {
        use copypasta::{ClipboardContext, ClipboardProvider};

        let text = "Round-trip test content";

        // Clear the clipboard before starting the test
        if let Ok(mut ctx) = ClipboardContext::new() {
            ctx.set_contents(String::new()).unwrap();
        } else {
            panic!("Failed to access clipboard context to clear contents.");
        }

        // Perform the copy-to-clipboard operation
        let result = copy_to_clipboard(text);
        assert!(result, "Expected clipboard copy to succeed, but it failed.");

        // Retrieve the clipboard content and verify it matches the original text
        if let Ok(mut ctx) = ClipboardContext::new() {
            let clipboard_content = ctx.get_contents().unwrap_or_default();
            assert_eq!(
                clipboard_content, text,
                "Clipboard content does not match. Expected '{}', got '{}'",
                text, clipboard_content
            );
        } else {
            panic!("Failed to access clipboard context to retrieve contents.");
        }
    }
}