use async_trait::async_trait;
use nimbus_vault_server_application::services::auth::{AuthService, errors::AuthServiceError};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::x509::{X509, X509Req};
use std::fs;

#[derive(Debug)]
pub struct AuthSettings {
    pub ca_cert_path: String,
    pub ca_key_path: String,
}

impl AuthSettings {
    pub fn new(ca_cert_path: String, ca_key_path: String) -> Self {
        Self {
            ca_cert_path,
            ca_key_path,
        }
    }
}

pub struct DefaultAuthService {
    ca_cert: X509,
    ca_key: PKey<openssl::pkey::Private>,
}

impl DefaultAuthService {
    pub fn new(settings: AuthSettings) -> Result<Self, AuthServiceError> {
        let ca_cert = Self::load_certificate(&settings.ca_cert_path)?;
        let ca_key = Self::load_private_key(&settings.ca_key_path)?;

        Ok(Self { ca_cert, ca_key })
    }

    fn load_certificate(path: &str) -> Result<X509, AuthServiceError> {
        let cert_data = fs::read_to_string(path).map_err(|e| {
            AuthServiceError::SigningFailed(format!("Failed to read certificate file: {}", e))
        })?;

        // Try PEM format first
        if cert_data.contains("-----BEGIN CERTIFICATE-----") {
            X509::from_pem(cert_data.as_bytes()).map_err(|e| {
                AuthServiceError::SigningFailed(format!("Failed to parse PEM certificate: {}", e))
            })
        } else {
            // Try DER format
            let der_data = fs::read(path).map_err(|e| {
                AuthServiceError::SigningFailed(format!("Failed to read certificate file: {}", e))
            })?;
            X509::from_der(&der_data).map_err(|e| {
                AuthServiceError::SigningFailed(format!("Failed to parse DER certificate: {}", e))
            })
        }
    }

    fn load_private_key(path: &str) -> Result<PKey<openssl::pkey::Private>, AuthServiceError> {
        let key_data = fs::read_to_string(path).map_err(|e| {
            AuthServiceError::SigningFailed(format!("Failed to read private key file: {}", e))
        })?;

        // Try PEM format first
        if key_data.contains("-----BEGIN PRIVATE KEY-----")
            || key_data.contains("-----BEGIN RSA PRIVATE KEY-----")
        {
            PKey::private_key_from_pem(key_data.as_bytes()).map_err(|e| {
                AuthServiceError::SigningFailed(format!("Failed to parse PEM private key: {}", e))
            })
        } else {
            // Try DER format
            let der_data = fs::read(path).map_err(|e| {
                AuthServiceError::SigningFailed(format!("Failed to read private key file: {}", e))
            })?;
            PKey::private_key_from_der(&der_data).map_err(|e| {
                AuthServiceError::SigningFailed(format!("Failed to parse DER private key: {}", e))
            })
        }
    }
}

#[async_trait]
impl AuthService for DefaultAuthService {
    async fn sign_certificate(&self, request: &Vec<u8>) -> Result<Vec<u8>, AuthServiceError> {
        // Parse the CSR from DER
        let csr = X509Req::from_der(request).map_err(|_| AuthServiceError::InvalidCsr)?;
        let pubkey = csr.public_key().map_err(|_| AuthServiceError::InvalidCsr)?;

        let mut builder = openssl::x509::X509Builder::new()
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_version(2)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;

        let serial = openssl::bn::BigNum::from_u32(2)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let serial = serial
            .to_asn1_integer()
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_serial_number(&serial)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_subject_name(csr.subject_name())
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_issuer_name(self.ca_cert.subject_name())
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_pubkey(&pubkey)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;

        let not_before = openssl::asn1::Asn1Time::days_from_now(0)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_not_before(&not_before)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;

        let days_from_now = openssl::asn1::Asn1Time::days_from_now(365)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_not_after(&days_from_now)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;

        builder
            .sign(&self.ca_key, MessageDigest::sha256())
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let cert = builder.build();
        let cert_der = cert
            .to_der()
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        Ok(cert_der)
    }

    async fn get_certificate_fingerprint(
        &self,
        certificate: &Vec<u8>,
    ) -> Result<String, AuthServiceError> {
        let cert = X509::from_der(certificate).map_err(|_| AuthServiceError::InvalidCertificate)?;
        let digest = cert
            .digest(MessageDigest::sha256())
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        Ok(hex::encode(digest))
    }

    async fn get_auth_by_fingerprint(
        &self,
        _certificate_fingerprint: &str,
    ) -> Result<
        (
            nimbus_vault_server_domain::entities::user::User,
            nimbus_vault_server_domain::entities::device::Device,
        ),
        AuthServiceError,
    > {
        todo!();
    }
}
