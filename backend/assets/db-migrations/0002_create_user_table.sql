CREATE TABLE user (
  id VARCHAR(255) NOT NULL,
  provider VARCHAR(30) NOT NULL,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(100) NULL,
  email_verified_at DATETIME NULL,
  recorded_at DATETIME NOT NULL,
  PRIMARY KEY(id)
);
