CREATE TABLE IF NOT EXISTS _user (
    _id                 SERIAL          PRIMARY KEY,
    _username           VARCHAR(36)     UNIQUE NOT NULL,
    _password_hash      TEXT            NOT NULL,
    _public_key         TEXT            NOT NULL,
    _created_at         TIMESTAMPZ      NOT NULL DEFAULT now (),
    _modified_at        TIMESTAMPZ      NOT NULL DEFAULT now (),
);
