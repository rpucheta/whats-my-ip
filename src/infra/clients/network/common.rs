use crate::infra::clients::network::{private, public};

/// Fetches both public and private IP addresses asynchronously.
///
/// # Returns
///
/// A tuple containing the public IP as the first element and the private IP as the second element.
///
/// # Errors
///
/// Returns an error if either the public or private IP cannot be fetched.
pub async fn get_public_and_private_ips() -> Result<(String, String), Box<dyn std::error::Error>> {
    let public_ip = public::get_public_ip().await?;
    let private_ip = private::get_private_ip().await?;
    Ok((public_ip, private_ip))
}

#[allow(dead_code)]
pub async fn get_public_and_private_ips_with_url(
    url: Option<&str>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let public_ip = public::get_public_ip_with_url(url).await;
    let private_ip = private::get_private_ip().await;

    match (public_ip, private_ip) {
        (Ok(public), Ok(private)) => Ok((public, private)),
        _ => Err("Failed to retrieve one or both IPs".into()),
    }
}