# rust_portfolio_api

#### Setting up mysql database
- Create database <br />
`CREATE DATABASE rust_portfolio;`

#### Setting up dev mysql user
- Get root access to mysql <br />
`sudo mysql`
- Create user <br />
`CREATE USER 'dev'@'localhost' IDENTIFIED BY 'password';`
- Grant user privileges <br />
`GRANT ALL PRIVILEGES ON *.* TO 'dev'@'localhost';`
- Login as user <br />
`mysql -u dev -p`
