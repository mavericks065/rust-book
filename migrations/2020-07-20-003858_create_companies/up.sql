-- Your SQL goes here
CREATE TABLE companies (
  id integer  PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  address TEXT,
  city VARCHAR ,
  abn TEXT,
  country VARCHAR
);
