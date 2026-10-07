use reqwest::Client;
use serde_json::json;

mod tests {
    use super::*;

    #[tokio::test]
    async fn valid_task_creation() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .post(format!("{}/task/create", base_url))
            .json(&json!({
                "title": "Test Task",
                "description": "A test description"
            }))
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn invalid_task_creation_empty_title() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .post(format!("{}/task/create", base_url))
            .json(&json!({
                "title": "",
                "description": "A test description"
            }))
            .send()
            .await
            .unwrap();
        assert!(!response.status().is_success());
    }

    #[tokio::test]
    async fn valid_user_creation() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .post(format!("{}/user/create", base_url))
            .json(&json!({
                "name": format!("testuser{}", rand::random_range(1..1000)),
                "password": "password123"
            }))
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn invalid_user_creation_short_password() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .post(format!("{}/user/create", base_url))
            .json(&json!({
                "name": "testuser2",
                "password": "short"
            }))
            .send()
            .await
            .unwrap();
        assert!(!response.status().is_success());
    }

    #[tokio::test]
    async fn valid_task_list() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .get(format!("{}/api/task/list", base_url))
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn valid_user_list() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .get(format!("{}/api/user/list", base_url))
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn invalid_task_patch_invalid_priority() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .patch(format!("{}/task/1", base_url))
            .json(&json!({
                "title": "Updated Task",
                "description": "Updated description",
                "priority": "invalid",
                "status": "new"
            }))
            .send()
            .await
            .unwrap();
        assert!(!response.status().is_success());
    }

    #[tokio::test]
    async fn invalid_task_patch_invalid_status() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .patch(format!("{}/task/1", base_url))
            .json(&json!({
                "title": "Updated Task",
                "description": "Updated description",
                "priority": "low",
                "status": "invalid"
            }))
            .send()
            .await
            .unwrap();
        assert!(!response.status().is_success());
    }

    #[tokio::test]
    async fn invalid_user_name_patch_empty_name() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .patch(format!("{}/user/1", base_url))
            .json(&json!({
                "name": ""
            }))
            .send()
            .await
            .unwrap();
        assert!(!response.status().is_success());
    }

    #[tokio::test]
    async fn invalid_user_name_patch_too_long_name() {
        let client = Client::new();
        let base_url = "http://127.0.0.1:3000";

        let response = client
            .patch(format!("{}/user/1", base_url))
            .json(&json!({
                "name": "a".repeat(51)
            }))
            .send()
            .await
            .unwrap();
        assert!(!response.status().is_success());
    }
}
