#!/bin/bash

# Generate Server Certificate
# This script generates a server certificate signed by the CA

set -e

# Default paths - use ~/.nimbus/certificates/ for macOS
NIMBUS_HOME="${NIMBUS_HOME:-$HOME/.nimbus}"
CERT_DIR="${CERT_DIR:-$NIMBUS_HOME/certificates}"
CA_CERT_PATH="${CA_CERT_PATH:-$CERT_DIR/ca-cert.pem}"
CA_KEY_PATH="${CA_KEY_PATH:-$CERT_DIR/ca-key.pem}"
SERVER_CERT_PATH="${SERVER_CERT_PATH:-$CERT_DIR/server-cert.pem}"
SERVER_KEY_PATH="${SERVER_KEY_PATH:-$CERT_DIR/server-key.pem}"
SERVER_CSR_PATH="${SERVER_CSR_PATH:-$CERT_DIR/server.csr}"

# Check if CA files exist
if [ ! -f "$CA_CERT_PATH" ] || [ ! -f "$CA_KEY_PATH" ]; then
    echo "Error: CA certificate or private key not found!"
    echo "Please run generate-ca.sh first."
    exit 1
fi

echo "Generating server certificate..."

# Create directories if they don't exist
mkdir -p "$(dirname "$SERVER_CERT_PATH")"
mkdir -p "$(dirname "$SERVER_KEY_PATH")"

# Generate server private key
echo "Generating server private key..."
openssl genrsa -out "$SERVER_KEY_PATH" 2048

# Generate server CSR
echo "Generating server CSR..."
openssl req -new -key "$SERVER_KEY_PATH" -out "$SERVER_CSR_PATH" \
    -subj "/C=US/ST=State/L=City/O=NimbusVault/OU=Server/CN=localhost"

# Sign server certificate with CA
echo "Signing server certificate with CA..."
openssl x509 -req -in "$SERVER_CSR_PATH" -CA "$CA_CERT_PATH" -CAkey "$CA_KEY_PATH" \
    -CAcreateserial -out "$SERVER_CERT_PATH" -days 365 -sha256

# Clean up CSR
rm -f "$SERVER_CSR_PATH"

echo "Server certificate generated successfully!"
echo "Files created:"
echo "  - $SERVER_CERT_PATH (Server Certificate)"
echo "  - $SERVER_KEY_PATH (Server Private Key)"

# Set proper permissions
chmod 600 "$SERVER_KEY_PATH"
chmod 644 "$SERVER_CERT_PATH"

echo "Certificate files created with appropriate permissions." 