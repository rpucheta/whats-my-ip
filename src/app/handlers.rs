use crate::infra::clients::network::{private, public, common};
use crate::infra::services::clipboard::copy_to_clipboard;
use crate::utils::formatters::{format_single_result, format_both_results};

pub async fn handle_public_ip() {
    if let Ok(public_ip) = public::get_public_ip().await {
        let copied = copy_to_clipboard(&public_ip);
        println!("{}", format_single_result("Public", &public_ip, copied));
    } else {
        eprintln!("[WhatsMyIp] Could not fetch public IP.");
    }
}

pub async fn handle_private_ip() {
    if let Ok(private_ip) = private::get_private_ip().await {
        let copied = copy_to_clipboard(&private_ip);
        println!("{}", format_single_result("Private", &private_ip, copied));
    } else {
        eprintln!("[WhatsMyIp] Could not fetch private IP.");
    }
}

pub async fn handle_both_ips() {
    if let Ok((public_ip, private_ip)) = common::get_public_and_private_ips().await {
        println!("{}", format_both_results(&public_ip, &private_ip));
    } else {
        eprintln!("[WhatsMyIp] Could not fetch both public and private IPs.");
    }
}