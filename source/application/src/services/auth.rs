use async_trait::async_trait;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn sign_certificate(&self, request: Vec<u8>) -> Vec<u8>;
    async fn get_certificate_fingerprint(&self, certificate: Vec<u8>) -> String;
}
