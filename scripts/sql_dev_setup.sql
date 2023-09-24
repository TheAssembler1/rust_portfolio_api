CREATE DATABASE portfolio_db;
CREATE USER 'dev_internal_user'@'localhost' IDENTIFIED BY 'password';
GRANT DROP, CREATE, INSERT, SELECT, DELETE, UPDATE ON portfolio_db.* TO 'dev_internal_user'@'localhost';
ALTER USER 'dev_internal_user'@'localhost' IDENTIFIED WITH mysql_native_password BY 'dev_password';
FLUSH PRIVILEGES;
