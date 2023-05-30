DROP DATABASE IF EXISTS rust_portfolio;
CREATE DATABASE rust_portfolio;
USE rust_portfolio;

CREATE TABLE test (
  id INT NOT NULL AUTO_INCREMENT,
  message VARCHAR(255) NOT NULL,

  PRIMARY KEY (id)
);

CREATE TABLE blog (
  id INT NOT NULL AUTO_INCREMENT,
  title VARCHAR(255) NOT NULL,
  html TEXT(65535) NOT NULL,

  PRIMARY KEY (id)
);

CREATE TABLE user (
  id VARCHAR(255) UNIQUE NOT NULL,
  email VARCHAR(320) UNIQUE NOT NULL,
  password VARCHAR(255) NOT NULL,
  salt VARCHAR(255) NOT NULL,

  PRIMARY KEY (id)
);
