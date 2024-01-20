CREATE TABLE IF NOT EXISTS AppUser (
    id uuid PRIMARY KEY,
    user_name TEXT NOT NULL,
    hash_password TEXT NOT NULL,
    display_name TEXT NOT NULL,
    is_admin BOOLEAN NOT NULL,
    email TEXT NOT NULL,
    mobile_phone TEXT NOT NULL,
    UNIQUE (user_name),
    UNIQUE (display_name)
);

CREATE INDEX IF NOT EXISTS idxUserEmail ON AppUser (email);

CREATE INDEX IF NOT EXISTS idxUserMobilePhone ON AppUser (mobile_phone);
