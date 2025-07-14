use async_trait::async_trait;

#[async_trait]
pub trait CryptoService: Send + Sync {
    async fn get_password_hash(&self, password: String) -> String;
}
