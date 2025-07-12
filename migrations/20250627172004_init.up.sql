CREATE TYPE device_type AS ENUM ('Linux', 'Windows', 'MacOS');

CREATE TABLE users (
    id varchar(26) PRIMARY KEY,
    username varchar(128) NOT NULL,
    password_hash varchar(128) NOT NULL,
    e2e_key_hash varchar(64) NOT NULL,
    encrypted_master_key varchar(256) NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);

CREATE TABLE devices (
    id varchar(26) PRIMARY KEY,
    user_id varchar(26) NOT NULL,
    name varchar(128) NOT NULL,
    device_type device_type NOT NULL,
    cert_fingerprint varchar NOT NULL,
    registered_at timestamptz NOT NULL,
    last_seen_at timestamptz NOT NULL,
    revoked_at timestamptz
);

ALTER TABLE devices
ADD CONSTRAINT fk_devices_user_id FOREIGN KEY (user_id) REFERENCES users (id);

CREATE INDEX idx_devices_cert_fingerprint ON devices (cert_fingerprint);

CREATE INDEX idx_devices_user_id ON devices (user_id);

CREATE INDEX idx_devices_user_id_revoked ON devices (user_id)
WHERE
    revoked_at IS NULL;