use local_ip_address;

/// Gets the private IP address of the current machine.
///
/// # Returns
///
/// A `Result` containing a `String` with the private IP address if successful, or a `Box` with a `dyn` `std::error::Error` trait object if an error occurs.
pub async fn get_private_ip() -> Result<String, Box<dyn std::error::Error>> {
    get_private_ip_with_provider(|| {
        local_ip_address::local_ip().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    })
        .await
}


pub async fn get_private_ip_with_provider<F>(provider: F) -> Result<String, Box<dyn std::error::Error>>
where
    F: Fn() -> Result<std::net::IpAddr, Box<dyn std::error::Error>>,
{
    let ip = provider()?;
    Ok(ip.to_string())
}