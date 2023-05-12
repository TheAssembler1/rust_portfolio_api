# rust_portfolio_api

#### Setting up dev mysql user
1. Get root access to mysql
`sudo mysql`<br>
2. Create user
`CREATE USER 'dev'@'localhost' IDENTIFIED BY 'password';`<br>
3. Grant user privileges
`GRANT ALL PRIVILEGES ON *.* TO 'dev'@'localhost';`<br>
4. Login as user
`mysql -u dev -p`<br>
