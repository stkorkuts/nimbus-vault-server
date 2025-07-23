#!/bin/bash

# Generate CA Certificate and Private Key
# This script generates a CA certificate and private key for NimbusVault

set -e

# Default paths - use ~/.nimbus/certificates/ for macOS
NIMBUS_HOME="${NIMBUS_HOME:-$HOME/.nimbus}"
CERT_DIR="${CERT_DIR:-$NIMBUS_HOME/certificates}"
CA_CERT_PATH="${CA_CERT_PATH:-$CERT_DIR/ca-cert.pem}"
CA_KEY_PATH="${CA_KEY_PATH:-$CERT_DIR/ca-key.pem}"

echo "Generating CA certificate and private key..."

# Create directories if they don't exist
mkdir -p "$(dirname "$CA_CERT_PATH")"
mkdir -p "$(dirname "$CA_KEY_PATH")"

# Generate CA private key
echo "Generating CA private key..."
openssl genrsa -out "$CA_KEY_PATH" 2048

# Generate CA certificate
echo "Generating CA certificate..."
openssl req -new -x509 -days 3650 -key "$CA_KEY_PATH" -out "$CA_CERT_PATH" \
    -subj "/C=US/ST=State/L=City/O=NimbusVault/OU=Certificate Authority/CN=NimbusVault CA"

echo "CA certificate and private key generated successfully!"
echo "Files created:"
echo "  - $CA_CERT_PATH (CA Certificate)"
echo "  - $CA_KEY_PATH (CA Private Key)"

# Set proper permissions
chmod 600 "$CA_KEY_PATH"
chmod 644 "$CA_CERT_PATH"

echo "Certificate files created with appropriate permissions." 