CREATE TYPE DeviceType AS ENUM ('Linux', 'Windows', 'MacOS');

CREATE TABLE Users (
    id varchar(26) PRIMARY KEY,
    username varchar(128) NOT NULL,
    password_hash varchar(128) NOT NULL,
    e2e_key_hash varchar(64) NOT NULL,
    encrypted_master_key varchar(256) NOT NULL,
    created_at timestamp NOT NULL,
    updated_at timestamp NOT NULL
);

CREATE TABLE Devices (
    id varchar(26) PRIMARY KEY,
    user_id varchar(26) NOT NULL,
    name varchar(128) NOT NULL,
    device_type DeviceType NOT NULL,
    cert_fingerprint varchar NOT NULL,
    registered_at timestamp NOT NULL,
    last_seen_at timestamp NOT NULL,
    revoked_at timestamp
);

ALTER TABLE Devices
ADD CONSTRAINT fk_devices_user_id FOREIGN KEY (user_id) REFERENCES Users (id);