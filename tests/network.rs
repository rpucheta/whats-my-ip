#[cfg(test)]
mod tests {
    use mockito::{Mock, Server};
    use whats_my_ip::infra::clients::network::{public, private,common};

    /// Tests public IP retrieval with mockito.
    #[tokio::test]
    async fn test_get_public_ip_success() {
        let mut server = Server::new_async().await;

        // Mock the external IP API
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("8.8.8.8")
            .create_async()
            .await;

        // Use the mock server's URL
        let url = server.url();
        let public_ip = public::get_public_ip_with_url(Some(&url)).await.unwrap();

        // Assert that the public IP is as expected
        assert_eq!(public_ip, "8.8.8.8");

        // Verify that the mock endpoint was called
        mock.assert_async().await;
    }

    /// Tests private IP retrieval using `local_ip_address`.
    #[tokio::test]
    async fn test_get_private_ip_success() {
        // Simulate getting the private IP
        let private_ip = private::get_private_ip().await;

        assert!(private_ip.is_ok(), "Failed to fetch private IP address");
        let private_ip = private_ip.unwrap();

        // Replace this with an expected private IP for your system or a pattern.
        println!("[Test Info] Private IP retrieved: {}", private_ip);
        assert!(!private_ip.is_empty(), "Private IP should not be empty");
    }

    /// Tests retrieval of both public and private IPs.
    #[tokio::test]
    async fn test_get_public_and_private_ips_success() {
        let mut server = Server::new_async().await;

        let mock: Mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("8.8.8.8")
            .create_async()
            .await;

        let url = server.url();
        let result = common::get_public_and_private_ips_with_url(Some(url.as_str())).await;

        assert!(result.is_ok(), "Failed to fetch public and private IPs");
        let (result_public_ip, result_private_ip) = result.unwrap();

        assert_eq!(result_public_ip, "8.8.8.8");
        assert!(!result_private_ip.is_empty(), "Private IP should not be empty");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_public_ip_timeout() {
        use httpmock::MockServer;
        use tokio::time::timeout;

        // Set up the mock server
        let server = MockServer::start();

        // Add a mock endpoint with a delay
        let mock = server.mock(|when, then| {
            when.method("GET").path("/");
            then.status(200).body("8.8.8.8").delay(std::time::Duration::from_secs(5)); // 5-second delay
        });

        // Simulate a timeout with a shorter duration than the server delay
        let result = timeout(
            std::time::Duration::from_secs(1), // Timeout duration
            public::get_public_ip_with_url(Some(&server.url("/"))),
        )
            .await;

        // Assert that a timeout error occurred
        assert!(result.is_err(), "Expected a timeout error but got a result");

        // Verify the mock was called
        mock.assert();
    }

    #[tokio::test]
    async fn test_get_public_ip_invalid_response() {
        let mut server = Server::new_async().await;

        // Mock an invalid response from the external IP API
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("invalid-ip")
            .create_async()
            .await;

        let url = server.url();
        let result = public::get_public_ip_with_url(Some(&url)).await;

        // Assert that the function does not return a valid IP
        assert!(result.is_err(), "Expected an error for invalid IP but got a result");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_public_ip_non_200_status() {
        let mut server = Server::new_async().await;

        // Mock a non-200 response
        let mock = server
            .mock("GET", "/")
            .with_status(500)
            .create_async()
            .await;

        let url = server.url();
        let result = public::get_public_ip_with_url(Some(&url)).await;

        // Assert that an error is returned for non-200 responses
        assert!(result.is_err(), "Expected an error for non-200 status code but got a result");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_public_ip_empty_body() {
        let mut server = Server::new_async().await;

        // Mock an empty response body
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_body("")
            .create_async()
            .await;

        let url = server.url();
        let result = public::get_public_ip_with_url(Some(&url)).await;

        // Assert that an error is returned for an empty body
        assert!(result.is_err(), "Expected an error for empty body but got a result");

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_get_public_ip_network_failure() {
        // Use an invalid URL to simulate a network failure
        let url = "http://invalid-url";

        let result = public::get_public_ip_with_url(Some(url)).await;

        // Assert that an error is returned for a network failure
        assert!(result.is_err(), "Expected an error for network failure but got a result");
    }

    #[tokio::test]
    async fn test_get_private_ip_failure() {
        let mock_provider = || Err("Simulated failure".into());

        let result = private::get_private_ip_with_provider(mock_provider).await;

        assert!(result.is_err(), "Expected an error for private IP retrieval failure but got a result");
    }

    #[tokio::test]
    async fn test_get_public_and_private_ips_partial_failure() {
        let mut server = Server::new_async().await;

        // Simulate failure for public IP
        let mock = server
            .mock("GET", "/")
            .with_status(500)
            .create_async()
            .await;

        let url = server.url();
        let result = common::get_public_and_private_ips_with_url(Some(url.as_str())).await;

        // Assert that an error is returned
        assert!(result.is_err(), "Expected an error for partial failure but got a result");

        mock.assert_async().await;
    }

    use assert_cmd::Command;

    #[tokio::test]
    async fn test_main_with_public_ip() {
        let mut cmd = Command::cargo_bin("whatsmyip").unwrap();

        // Pass arguments to the binary
        cmd.arg("-public");

        // Mock server behavior if needed, otherwise test normally
        // Ensure the binary fetches the public IP
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Public IP:"));
    }

}