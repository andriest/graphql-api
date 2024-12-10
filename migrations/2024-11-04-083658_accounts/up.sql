CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    nickname TEXT NOT NULL DEFAULT '',
    full_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone_num TEXT NOT NULL,
    activated_at TIMESTAMP,
    ts TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
