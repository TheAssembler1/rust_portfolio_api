# portfolio_api

#### Diesel cli install
`sudo apt install default-libmysqlclient-dev`
`cargo install diesel_cli --no-default-features --features mysql`

#### Open mysql cli
- Get root access to mysql cli <br />
`sudo mysql`

#### Setting up mysql database
- Create database <br />
`CREATE DATABASE portfolio_db;`

#### Setting up dev mysql user
- Create user internal developer <br />
`CREATE USER 'dev_internal_user'@'localhost' IDENTIFIED BY 'dev_password';`
- Grant user internal developer privileges <br />
`GRANT ALL PRIVILEGES ON rust_portfolio.* TO 'dev_internal_user'@'localhost';`
- Login as user <br />
`mysql -u dev -p`
