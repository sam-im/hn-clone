CREATE TABLE IF NOT EXISTS _user (
    _id             SERIAL          PRIMARY KEY,
    _username       VARCHAR(36)     UNIQUE NOT NULL,
    _password_hash  VARCHAR(128)    NOT NULL,                   -- Argon2 PHC string is 97 chars w/default config
    _about          VARCHAR(256),
    _public_key     TEXT,
    _created_at     TIMESTAMPTZ     NOT NULL DEFAULT now (),
    _modified_at    TIMESTAMPTZ     NOT NULL DEFAULT now ()
);

CREATE TABLE IF NOT EXISTS _item (
    _id             SERIAL          PRIMARY KEY,
    _type           VARCHAR(12)     NOT NULL,
    _created_at     TIMESTAMPTZ     NOT NULL DEFAULT now (),
    _modified_at    TIMESTAMPTZ     NOT NULL DEFAULT now (),
    _owner          INTEGER         NOT NULL,
    CONSTRAINT fk_item_owner
        FOREIGN KEY (_owner) REFERENCES _user(_id)
        ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _post(
    _id             INTEGER         PRIMARY KEY,
    _title          TEXT            NOT NULL,
    _content        TEXT            NOT NULL,
    CONSTRAINT fk_post_id
        FOREIGN KEY (_id) REFERENCES _item(_id)
        ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _comment (
    _id             INTEGER         PRIMARY KEY,
    _content        TEXT            NOT NULL,
    _parent         INTEGER         NOT NULL,
    CONSTRAINT fk_comment_id
        FOREIGN KEY (_id) REFERENCES _item(_id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT fk_comment_parent
        FOREIGN KEY (_parent) REFERENCES _item(_id)
        ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS _upvote (
    _item           INTEGER         NOT NULL,
    _user           INTEGER         NOT NULL,
    PRIMARY KEY (_item, _user),
    CONSTRAINT fk_upvote_item
        FOREIGN KEY (_item) REFERENCES _item(_id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT fk_upvote_user
        FOREIGN KEY (_user) REFERENCES _user(_id)
        ON UPDATE CASCADE ON DELETE CASCADE
);

