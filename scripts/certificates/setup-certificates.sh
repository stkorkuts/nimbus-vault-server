#!/bin/bash

# Setup Certificates for NimbusVault
# This script generates CA and server certificates

set -e

# Default paths - use ~/.nimbus/certificates/ for macOS
NIMBUS_HOME="${NIMBUS_HOME:-$HOME/.nimbus}"
CERT_DIR="${CERT_DIR:-$NIMBUS_HOME/certificates}"
CA_CERT_PATH="${CA_CERT_PATH:-$CERT_DIR/ca-cert.pem}"
CA_KEY_PATH="${CA_KEY_PATH:-$CERT_DIR/ca-key.pem}"
SERVER_CERT_PATH="${SERVER_CERT_PATH:-$CERT_DIR/server-cert.pem}"
SERVER_KEY_PATH="${SERVER_KEY_PATH:-$CERT_DIR/server-key.pem}"

echo "Setting up certificates for NimbusVault..."

# Check if OpenSSL is available
if ! command -v openssl &> /dev/null; then
    echo "Error: OpenSSL is not installed or not in PATH"
    echo "Please install OpenSSL and try again."
    exit 1
fi

# Create certificate directory if it doesn't exist
mkdir -p "$CERT_DIR"

# Generate CA certificate if it doesn't exist
if [ ! -f "$CA_CERT_PATH" ] || [ ! -f "$CA_KEY_PATH" ]; then
    echo "CA certificate not found. Generating CA certificate..."
    ./scripts/certificates/generate-ca.sh
else
    echo "CA certificate already exists."
fi

# Generate server certificate if it doesn't exist
if [ ! -f "$SERVER_CERT_PATH" ] || [ ! -f "$SERVER_KEY_PATH" ]; then
    echo "Server certificate not found. Generating server certificate..."
    ./scripts/certificates/generate-server-cert.sh
else
    echo "Server certificate already exists."
fi

echo ""
echo "Certificate setup completed!"
echo ""
echo "Certificate files:"
echo "  - CA Certificate: $CA_CERT_PATH"
echo "  - CA Private Key: $CA_KEY_PATH"
echo "  - Server Certificate: $SERVER_CERT_PATH"
echo "  - Server Private Key: $SERVER_KEY_PATH"
echo ""
echo "You can now start the NimbusVault server." 