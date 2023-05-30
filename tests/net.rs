#[cfg(test)]
mod net_integration_tests {
    use rubin::net::client::RubinClient;
    use rubin::net::server::start;

    async fn sleep(duration: u64) {
        tokio::time::sleep(tokio::time::Duration::from_millis(duration)).await;
    }

    #[tokio::test]
    async fn connects_to_server_and_performs_successful_request() {
        let server = tokio::spawn(start("127.0.0.1", 9876));
        sleep(1000).await;

        let client = RubinClient::new("127.0.0.1", 9876);
        let response = client.insert_string("user:1000", "value1").await.unwrap();
        assert!(&response == "OK");

        server.abort();
    }

    #[tokio::test]
    async fn gets_value_from_the_server() {
        let server = tokio::spawn(start("127.0.0.1", 9877));
        sleep(1000).await;

        let client = RubinClient::new("127.0.0.1", 9877);
        client.insert_string("user:1000", "value1").await.unwrap();

        sleep(500).await;
        let response = client.get_string("user:1000").await.unwrap();
        assert_eq!(&response, "value1");

        server.abort();
    }

    #[tokio::test]
    async fn gets_an_error_from_the_server() {
        let server = tokio::spawn(start("127.0.0.1", 9878));
        sleep(1000).await;

        let client = RubinClient::new("127.0.0.1", 9878);
        let response = client.insert_string("user:1000", "").await.unwrap();

        assert_eq!(&response, "invalid message");

        server.abort();
    }
}
