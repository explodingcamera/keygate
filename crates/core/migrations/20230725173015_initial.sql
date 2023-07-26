-- Enable foreign key constraints
PRAGMA foreign_keys = ON;

CREATE TABLE Identity (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    last_active TIMESTAMP NOT NULL,

    username VARCHAR(255) UNIQUE,
    primary_email VARCHAR(255),

    password_hash VARCHAR(255),

    FOREIGN KEY (primary_email) REFERENCES Email (email)
);

CREATE TABLE Email (
    email TEXT PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    verified BOOLEAN CHECK (verified IN (0, 1)) NOT NULL,
    last_verification_request TIMESTAMP,
    verification_code VARCHAR(255),
    verification_code_expires_at TIMESTAMP,
    identity_id VARCHAR(36) NOT NULL,

    FOREIGN KEY (identity_id) REFERENCES Identity (id)
);

CREATE TABLE LinkedAccount (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    provider_id VARCHAR(255) NOT NULL
);

CREATE TABLE LoginProcess (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    identity_id VARCHAR(36) NOT NULL,
    ip_address VARCHAR(255),
    expires_at TIMESTAMP,
    completed BOOLEAN CHECK (completed IN (0, 1)) NOT NULL,
    current_step VARCHAR(255) NOT NULL,
    magic_link VARCHAR(255)
);

CREATE TABLE PublicKey (
    id INTEGER PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    key_type VARCHAR(255) NOT NULL,
    node_id VARCHAR(255) NOT NULL,
    valid_until TIMESTAMP NOT NULL,
    revoked_at TIMESTAMP,
    public_key BLOB NOT NULL
);

CREATE TABLE Meta (
    key VARCHAR(255) PRIMARY KEY NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    value VARCHAR(255) NOT NULL,
    value_date TIMESTAMP,
    value_int INTEGER,
    value_bool BOOLEAN CHECK (value_bool IN (0, 1)),
    value_byte BLOB
);

CREATE TABLE AuditLog (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    identity_id VARCHAR(36) NOT NULL,
    session_id VARCHAR(36),
    node_id VARCHAR(255) NOT NULL,
    action VARCHAR(255) NOT NULL,
    target_id VARCHAR(255),
    target_type VARCHAR(255),
    data VARCHAR(255)
);

CREATE TABLE Session (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP NOT NULL,
    revoked_at TIMESTAMP,
    initial_ip_address VARCHAR(255),
    node_id VARCHAR(255) NOT NULL,
    refresh_token VARCHAR(255) NOT NULL,
    identity_id VARCHAR(36) NOT NULL
);

CREATE TABLE APIKey (
    key VARCHAR(255) PRIMARY KEY NOT NULL,
    identity_id VARCHAR(36),
    name VARCHAR(255),
    target VARCHAR(255) NOT NULL,
    audience VARCHAR(255) NOT NULL,
    public BOOLEAN CHECK (public IN (0, 1)) NOT NULL,
    hostnames VARCHAR(255) NOT NULL
);

CREATE TABLE Application (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    settings TEXT NOT NULL
);
