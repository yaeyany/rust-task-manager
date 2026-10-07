use rust_task_manager::handlers::{html_handler, redirect_to_login};

mod tests {
    use super::*;

    #[tokio::test]
    async fn html_handler_valid_file() {
        let result = html_handler("templates/login.html").await;
        assert!(result.0.contains("<title>Login</title>"));
    }

    #[tokio::test]
    async fn html_handler_missing_file() {
        let result = html_handler("templates/nonexistent.html").await;
        assert!(result.0.contains("500 Internal Server Error"));
    }

    #[tokio::test]
    async fn redirect_to_login_returns_redirect() {
        let result = redirect_to_login().await;
        assert_eq!(result.status_code(), 307);
    }
}
