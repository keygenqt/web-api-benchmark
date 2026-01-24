CREATE TABLE users (
    id          SERIAL PRIMARY KEY,
    email       VARCHAR(255) UNIQUE NOT NULL,
    fname       VARCHAR(255) NOT NULL,
    lname       VARCHAR(255) NOT NULL,
    paswd       VARCHAR(255),
    created_at  TIMESTAMP NOT NULL
);
