-- Your SQL goes here

CREATE TABLE employees
(
    id         integer PRIMARY KEY,
    email      VARCHAR NOT NULL,
    firsname   VARCHAR NOT NULL,
    lastname   VARCHAR NOT NULL,
    address    TEXT,
    post_code  integer,
    city       VARCHAR,
    country    VARCHAR,
    dob        TIMESTAMP,
    salary     integer,
    company_id integer NOT NULL,
    created_at TIMESTAMP
);

CREATE TABLE companies
(
    id          integer PRIMARY KEY,
    name        VARCHAR   NOT NULL,
    description TEXT      NOT NULL,
    address     TEXT      NOT NULL,
    post_code   integer   NOT NULL,
    city        VARCHAR   NOT NULL,
    country     VARCHAR   NOT NULL,
    abn         TEXT,
    ceo_id      integer REFERENCES employees (id),
    created_at  TIMESTAMP NOT NULL DEFAULT NOW()
);
