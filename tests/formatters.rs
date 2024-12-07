use whats_my_ip::utils::formatters::{format_single_result, format_both_results};

#[test]
fn test_format_single_result_public_with_copy() {
    let result = format_single_result("Public", "8.8.8.8", true);
    assert_eq!(
        result,
        "[WhatsMyIp] 🌍 Public IP: (copied to clipboard) 🗒️✅ -> 8.8.8.8" // No space between 🗒️ and ✅
    );
}

#[test]
fn test_format_single_result_private_without_copy() {
    let result = format_single_result("Private", "192.168.1.1", false);
    assert_eq!(
        result,
        "[WhatsMyIp] 🏠 Private IP: ⚠️ -> 192.168.1.1"
    );
}

#[test]
fn test_format_both_results() {
    let result = format_both_results("8.8.8.8", "192.168.1.1");
    assert_eq!(
        result,
        "[WhatsMyIp] 🌍 Public IP: 8.8.8.8\n[WhatsMyIp] 🏠 Private IP: 192.168.1.1\n[WhatsMyIp] Note: To copy an IP to the clipboard, use `-public` or `-private`."
    );
}