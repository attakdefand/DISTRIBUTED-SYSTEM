#[cfg(test)]
mod integration_tests {
    use reqwest;
    use serde_json::Value;
    use std::time::Duration;

    #[tokio::test]
    async fn test_user_service_health() {
        // Give services time to start
        tokio::time::sleep(Duration::from_secs(2)).await;

        let client = reqwest::Client::new();
        let res = client
            .get("http://localhost:3001/health")
            .send()
            .await
            .expect("Failed to send request to user service");

        assert_eq!(res.status(), 200);
        let body = res.text().await.expect("Failed to get response body");
        assert_eq!(body, "User service is healthy");
    }

    #[tokio::test]
    async fn test_order_service_health() {
        // Give services time to start
        tokio::time::sleep(Duration::from_secs(2)).await;

        let client = reqwest::Client::new();
        let res = client
            .get("http://localhost:3002/health")
            .send()
            .await
            .expect("Failed to send request to order service");

        assert_eq!(res.status(), 200);
        let body = res.text().await.expect("Failed to get response body");
        assert_eq!(body, "Order service is healthy");
    }

    #[tokio::test]
    async fn test_gateway_health() {
        // Give services time to start
        tokio::time::sleep(Duration::from_secs(2)).await;

        let client = reqwest::Client::new();
        let res = client
            .get("http://localhost:3000/health")
            .send()
            .await
            .expect("Failed to send request to gateway");

        assert_eq!(res.status(), 200);
        let body = res.text().await.expect("Failed to get response body");
        assert_eq!(body, "Gateway is healthy");
    }

    #[tokio::test]
    async fn test_web_bff_health() {
        // Give services time to start
        tokio::time::sleep(Duration::from_secs(2)).await;

        let client = reqwest::Client::new();
        let res = client
            .get("http://localhost:3003/health")
            .send()
            .await
            .expect("Failed to send request to web BFF");

        assert_eq!(res.status(), 200);
        let body = res.text().await.expect("Failed to get response body");
        assert_eq!(body, "Web BFF is healthy");
    }

    #[tokio::test]
    async fn test_gateway_proxy_to_user_service() {
        // Give services time to start
        tokio::time::sleep(Duration::from_secs(2)).await;

        let client = reqwest::Client::new();
        let res = client
            .get("http://localhost:3000/api/users/123")
            .send()
            .await
            .expect("Failed to send request to gateway");

        // Should get a response from the user service (even if it's a mock)
        assert_eq!(res.status(), 200);
    }

    #[tokio::test]
    async fn test_gateway_proxy_to_order_service() {
        // Give services time to start
        tokio::time::sleep(Duration::from_secs(2)).await;

        let client = reqwest::Client::new();
        let res = client
            .get("http://localhost:3000/api/orders/123")
            .send()
            .await
            .expect("Failed to send request to gateway");

        // Should get a response from the order service (even if it's a mock)
        assert_eq!(res.status(), 200);
    }
}