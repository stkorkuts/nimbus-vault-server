DROP INDEX IF EXISTS idx_devices_cert_fingerprint;

DROP INDEX IF EXISTS idx_devices_user_id;

DROP INDEX IF EXISTS idx_devices_user_id_revoked;

ALTER TABLE devices DROP CONSTRAINT fk_devices_user_id;

DROP TABLE IF EXISTS devices;

DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS device_type;