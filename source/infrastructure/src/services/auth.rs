use async_trait::async_trait;
use nimbus_vault_server_application::services::auth::{AuthService, errors::AuthServiceError};
use nimbus_vault_server_domain::entities::{device::Device, user::User};
use openssl::asn1::Asn1Time;
use openssl::bn::BigNum;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::X509Builder;
use openssl::x509::X509NameBuilder;
use openssl::x509::{X509, X509Req};

pub struct DefaultAuthService {}

#[async_trait]
impl AuthService for DefaultAuthService {
    async fn sign_certificate(&self, request: &Vec<u8>) -> Result<Vec<u8>, AuthServiceError> {
        // For demonstration, use a hardcoded CA key and cert. Replace with secure storage in production.
        let ca_key =
            Rsa::generate(2048).map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let ca_pkey =
            PKey::from_rsa(ca_key).map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let mut name_builder =
            X509NameBuilder::new().map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        name_builder
            .append_entry_by_nid(Nid::COMMONNAME, "NimbusVault CA")
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let ca_name = name_builder.build();
        let mut ca_builder =
            X509Builder::new().map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        ca_builder
            .set_version(2)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let serial =
            BigNum::from_u32(1).map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let serial = serial
            .to_asn1_integer()
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        ca_builder
            .set_serial_number(&serial)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        ca_builder
            .set_subject_name(&ca_name)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        ca_builder
            .set_issuer_name(&ca_name)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        ca_builder
            .set_pubkey(&ca_pkey)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let not_before = Asn1Time::days_from_now(0)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        ca_builder
            .set_not_before(&not_before)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let days_from_now = Asn1Time::days_from_now(365)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        ca_builder
            .set_not_after(&days_from_now)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        ca_builder
            .sign(&ca_pkey, MessageDigest::sha256())
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let ca_cert = ca_builder.build();

        // Parse the CSR from DER
        let csr = X509Req::from_der(request).map_err(|_| AuthServiceError::InvalidCsr)?;
        let pubkey = csr.public_key().map_err(|_| AuthServiceError::InvalidCsr)?;
        let mut builder =
            X509Builder::new().map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_version(2)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let serial =
            BigNum::from_u32(2).map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
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
            .set_issuer_name(ca_cert.subject_name())
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_pubkey(&pubkey)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let not_before = Asn1Time::days_from_now(0)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_not_before(&not_before)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        let days_from_now = Asn1Time::days_from_now(365)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .set_not_after(&days_from_now)
            .map_err(|e| AuthServiceError::SigningFailed(e.to_string()))?;
        builder
            .sign(&ca_pkey, MessageDigest::sha256())
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
        certificate_fingerprint: &str,
    ) -> Result<(User, Device), AuthServiceError> {
        todo!();
    }
}
