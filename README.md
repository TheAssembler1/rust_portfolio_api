# rust_portfolio_api

#### Setting up dev mysql user
1. Get root access to mysql
`sudo mysql`
2. Create user
`CREATE USER 'dev'@'localhost' IDENTIFIED BY 'password';`
3. Grant user privileges
`GRANT ALL PRIVILEGES ON *.* TO 'dev'@'localhost';`
4. Login as user
`mysql -u dev -p`
