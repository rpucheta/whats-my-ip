use reqwest;
use local_ip_address;
use copypasta::{ClipboardContext, ClipboardProvider};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-public" => {
                if let Ok(public_ip) = get_public_ip().await {
                    let copied = copy_to_clipboard(&public_ip);
                    println!(
                        "[WhatsMyIp] 🌍 Public IP: {} {} -> {}",
                        if copied { "(copied to clipboard) 🗒️" } else { "" },
                        if copied { "✅" } else { "⚠️" },
                        public_ip
                    );
                } else {
                    eprintln!("[WhatsMyIp] Could not fetch public IP.");
                }
            }

            "-private" => {
                if let Ok(private_ip) = get_private_ip().await {
                    let copied = copy_to_clipboard(&private_ip);
                    println!(
                        "[WhatsMyIp] 🏠 Private IP: {} {} -> {}",
                        if copied { "(copied to clipboard) 🗒️" } else { "" },
                        if copied { "✅" } else { "⚠️" },
                        private_ip
                    );
                } else {
                    eprintln!("[WhatsMyIp] Could not fetch private IP.");
                }
            }

            _ => eprintln!("[WhatsMyIp] Invalid option. Use -public or -private."),
        }
    }else {
        if let Ok((public_ip, private_ip)) = get_public_and_private_ips().await {
            println!("[WhatsMyIp] 🌍 Public IP: {}", public_ip);
            println!("[WhatsMyIp] 🏠 Private IP: {}", private_ip);
            println!("[WhatsMyIp] Note: To copy an IP to the clipboard, use `-public` or `-private`.");
        } else {
            eprintln!("[WhatsMyIp] Could not fetch both public and private IPs.");
        }
    }
}


async fn get_public_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.ipify.org?format=text").await?;
    let ip = response.text().await?;
    Ok(ip)
}

async fn get_private_ip() -> Result<String, Box<dyn std::error::Error>> {
    let ip = local_ip_address::local_ip()?;
    Ok(ip.to_string())
}

async fn get_public_and_private_ips() -> Result<(String, String), Box<dyn std::error::Error>> {
    let public_ip = get_public_ip().await?;
    let private_ip = get_private_ip().await?;
    Ok((public_ip, private_ip))
}

fn copy_to_clipboard(text: &str) -> bool {
    if let Ok(mut ctx) = ClipboardContext::new() {
        if ctx.set_contents(text.to_owned()).is_ok() {
            return true;
        }
    }
    false
}