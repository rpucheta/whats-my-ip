pub fn format_single_result(label: &str, ip: &str, copied: bool) -> String {
    format!(
        "[WhatsMyIp] {} {} IP: {}{} -> {}",
        if label=="Public" { "🌍" } else { "🏠" },
        label,
        if copied { "(copied to clipboard) 🗒️" } else { "" },
        if copied { "✅" } else { "⚠️" },
        ip
    )
}

pub fn format_both_results(public_ip: &str, private_ip: &str) -> String {
    format!(
        "[WhatsMyIp] 🌍 Public IP: {}\n[WhatsMyIp] 🏠 Private IP: {}\n[WhatsMyIp] Note: To copy an IP to the clipboard, use `-public` or `-private`.",
        public_ip, private_ip
    )
}