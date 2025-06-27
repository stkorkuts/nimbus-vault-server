CREATE TYPE DeviceType AS ENUM ('Android', 'iOS', 'Linux', 'Windows', 'macOS');
CREATE TYPE FolderOperationType AS ENUM ('Create', 'Rename', 'Move', 'Delete');
CREATE TYPE FileOperationType AS ENUM ('Create', 'Rename', 'Move', 'Modify', 'Delete');
CREATE TYPE FileVersionSyncStatus AS ENUM ('InProgress', 'Succeed', 'Failed');
CREATE TYPE StorageType AS ENUM ('Local', 'S3');

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
    user_id varchar(26),
    name varchar(128) NOT NULL,
    device_type DeviceType NOT NULL,
    cert_fingerprint varchar NOT NULL,
    registered_at timestamp NOT NULL,
    last_seen_at timestamp NOT NULL,
    revoked_at timestamp
);

CREATE TABLE FolderOperations (
    id varchar(26) PRIMARY KEY,
    user_id varchar(26),
    device_id varchar(26),
    type FolderOperationType NOT NULL,
    data jsonb NOT NULL,
    timestamp timestamp NOT NULL
);

CREATE TABLE FileOperations (
    id varchar(26) PRIMARY KEY,
    user_id varchar(26),
    device_id varchar(26),
    type FileOperationType NOT NULL,
    data jsonb NOT NULL,
    timestamp timestamp NOT NULL
);

CREATE TABLE Folders (
    id varchar(26) PRIMARY KEY,
    user_id varchar(26),
    parent_id varchar(26),
    encrypted_name varchar(256) NOT NULL,
    created_at timestamp NOT NULL,
    updated_at timestamp NOT NULL,
    deleted_at timestamp
);

CREATE TABLE Files (
    id varchar(26) PRIMARY KEY,
    user_id varchar(26),
    parent_id varchar(26),
    latest_version_id varchar(26),
    encrypted_metadata jsonb NOT NULL,
    deleted_at timestamp
);

CREATE TABLE FileVersions (
    id varchar(26) PRIMARY KEY,
    file_id varchar(26),
    checksum bytea,
    size integer,
    sync_status FileVersionSyncStatus NOT NULL,
    sync_device_id varchar(26),
    created_at timestamp NOT NULL,
    deleted_at timestamp
);

CREATE TABLE FileVersionChunks (
    version_id varchar(26),
    index integer,
    chunk_id varchar(26),
    PRIMARY KEY (version_id, index)
);

CREATE TABLE FileChunks (
    id varchar(26) PRIMARY KEY,
    size integer NOT NULL,
    checksum bytea NOT NULL,
    ref_count integer NOT NULL,
    storage_type StorageType NOT NULL,
    storage_path varchar(1024) NOT NULL,
    created_at timestamp NOT NULL
);

ALTER TABLE Devices ADD CONSTRAINT fk_devices_user_id FOREIGN KEY (user_id) REFERENCES Users(id);
ALTER TABLE FolderOperations ADD CONSTRAINT fk_folderoperations_user_id FOREIGN KEY (user_id) REFERENCES Users(id);
ALTER TABLE FolderOperations ADD CONSTRAINT fk_folderoperations_device_id FOREIGN KEY (device_id) REFERENCES Devices(id);
ALTER TABLE FileOperations ADD CONSTRAINT fk_fileoperations_user_id FOREIGN KEY (user_id) REFERENCES Users(id);
ALTER TABLE FileOperations ADD CONSTRAINT fk_fileoperations_device_id FOREIGN KEY (device_id) REFERENCES Devices(id);
ALTER TABLE Folders ADD CONSTRAINT fk_folders_user_id FOREIGN KEY (user_id) REFERENCES Users(id);
ALTER TABLE Folders ADD CONSTRAINT fk_folders_parent_id FOREIGN KEY (parent_id) REFERENCES Folders(id);
ALTER TABLE Files ADD CONSTRAINT fk_files_user_id FOREIGN KEY (user_id) REFERENCES Users(id);
ALTER TABLE Files ADD CONSTRAINT fk_files_parent_id FOREIGN KEY (parent_id) REFERENCES Folders(id);
ALTER TABLE Files ADD CONSTRAINT fk_files_latest_version_id FOREIGN KEY (latest_version_id) REFERENCES FileVersions(id);
ALTER TABLE FileVersions ADD CONSTRAINT fk_fileversions_file_id FOREIGN KEY (file_id) REFERENCES Files(id);
ALTER TABLE FileVersions ADD CONSTRAINT fk_fileversions_sync_device_id FOREIGN KEY (sync_device_id) REFERENCES Devices(id);
ALTER TABLE FileVersionChunks ADD CONSTRAINT fk_fileversionchunks_version_id FOREIGN KEY (version_id) REFERENCES FileVersions(id);
ALTER TABLE FileVersionChunks ADD CONSTRAINT fk_fileversionchunks_chunk_id FOREIGN KEY (chunk_id) REFERENCES FileChunks(id);