-- Your SQL goes here

CREATE TABLE employees (
  id integer  PRIMARY KEY,
  email VARCHAR NOT NULL,
  firsname VARCHAR NOT NULL,
  lastname VARCHAR NOT NULL,
  address TEXT,
  post_code integer,
  city VARCHAR,
  country VARCHAR,
  dob TIMESTAMP ,
  salary integer,
  company_id integer NOT NULL,
  created_at TIMESTAMP
);

CREATE TABLE companies (
  id integer  PRIMARY KEY,
  name VARCHAR NOT NULL,
  description TEXT,
  address TEXT,
  post_code integer,
  city VARCHAR ,
  country VARCHAR,
  abn TEXT,
  ceo_id integer REFERENCES employees(id),
  created_at TIMESTAMP
);
