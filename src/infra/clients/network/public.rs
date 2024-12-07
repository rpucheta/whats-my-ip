use reqwest;
use crate::infra::clients::network::constants::IP_API_URL;
use std::net::IpAddr;
use std::error::Error;
/// Gets the public IP address of the current machine.
///
/// # Arguments
///
/// * `url` - An optional custom URL for testing purposes.
///
/// # Returns
///
/// A `Result` containing a `String` with the public IP address if successful, or a `reqwest::Error` if an error occurs.
pub async fn get_public_ip_with_url(url: Option<&str>) -> Result<String, Box<dyn Error>> {
    let api_url = url.unwrap_or(IP_API_URL);
    let response = reqwest::get(api_url).await?;

    let ip = response.text().await?;

    // Validate the IP address
    if ip.parse::<IpAddr>().is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid IP format",
        )));
    }

    Ok(ip)
}

/// Wrapper for production use.
pub async fn get_public_ip() -> Result<String, Box<dyn std::error::Error>> {
    get_public_ip_with_url(None).await
}