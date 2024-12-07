use copypasta::{ClipboardContext, ClipboardProvider};

/// Copy the given text to the clipboard.
///
/// # Arguments
///
/// * `text` - A string slice that holds the text to be copied.
///
/// # Returns
///
/// A boolean indicating if the text was successfully copied to the clipboard.
pub fn copy_to_clipboard(text: &str) -> bool {
    if let Ok(mut ctx) = ClipboardContext::new() {
        if ctx.set_contents(text.to_owned()).is_ok() {
            return true;
        }
    }
    false
}