CREATE TABLE users_axum (
    id          SERIAL PRIMARY KEY,
    email       VARCHAR(255) UNIQUE NOT NULL,
    fname       VARCHAR(255) NOT NULL,
    lname       VARCHAR(255) NOT NULL,
    pwd_hash    VARCHAR(255) NOT NULL,
    create_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);